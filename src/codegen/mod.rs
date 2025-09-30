use crate::syntax::ast;
use cranelift_codegen::ir::types;
use cranelift_codegen::ir::{AbiParam, InstBuilder};
use cranelift_codegen::settings::{self};
use cranelift_codegen::verifier::verify_function;
use cranelift_codegen::Context;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

// use target_lexicon::Triple; // Reserved for future configurable targets

use crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration;
use crate::syntax::nodes::declarations::{
    ClassBodyDeclaration, ClassDeclaration, MethodDeclaration,
};
use crate::syntax::nodes::types::{PrimitiveType, Type};

// Trait for AST nodes that can be compiled
pub trait Compilable {
    // For now, pass main parts of the CodeGenerator mutably and an optional fully-qualified class name.
    fn compile_node(
        &self,
        class_name: Option<String>, // Fully-qualified class name if available
        module: &mut ObjectModule,
        builder_context: &mut FunctionBuilderContext,
        context: &mut Context,
    ) -> Result<(), String>;
}

// Implement Compilable for ClassDeclaration
impl Compilable for ClassDeclaration {
    fn compile_node(
        &self,
        parent_namespace: Option<String>,
        module: &mut ObjectModule,
        builder_context: &mut FunctionBuilderContext,
        context: &mut Context,
    ) -> Result<(), String> {
        log::info!("Compiling class: {}", self.name);
        let class_fqn = if let Some(ns) = parent_namespace {
            format!("{}.{}", ns, self.name.name)
        } else {
            self.name.name.clone()
        };
        for member in &self.body_declarations {
            match member {
                ClassBodyDeclaration::Method(method_decl) => {
                    // Pass the fully-qualified class name as context
                    method_decl.compile_node(
                        Some(class_fqn.clone()),
                        module,
                        builder_context,
                        context,
                    )?;
                }
                ClassBodyDeclaration::Field(field_decl) => {
                    // TODO: Handle field declarations (e.g., allocate space in constructor?)
                    log::info!(
                        "Skipping field declaration (in trait impl): {}",
                        field_decl.name
                    );
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// Implement Compilable for MethodDeclaration
impl Compilable for MethodDeclaration {
    fn compile_node(
        &self,
        class_name: Option<String>,
        module: &mut ObjectModule,
        builder_context: &mut FunctionBuilderContext,
        context: &mut Context,
    ) -> Result<(), String> {
        let current_class_name = class_name.ok_or_else(|| "Method compilation requires class context".to_string())?;
        log::info!(
            "Compiling method: {}.{}",
            current_class_name,
            self.name // Use self.name directly
        );

        let return_type = map_type_stub(&self.return_type); // Use stub for now
        let mut sig = module.make_signature();
        if let Some(ret_type) = return_type {
            sig.returns.push(AbiParam::new(ret_type));
        }

        for param in &self.parameters {
            if let Some(param_type) = map_type_stub(&param.parameter_type) {
                // Use stub for now
                sig.params.push(AbiParam::new(param_type));
            } else {
                return Err(format!(
                    "Unsupported parameter type: {:?}\nIn method: {}.{}",
                    param.parameter_type, current_class_name, self.name
                ));
            }
        }

        let func_name = format!("{}.{}", current_class_name, self.name);
        let func_id = module
            .declare_function(&func_name, Linkage::Export, &sig)
            .map_err(|e| format!("Failed to declare function {}: {}", func_name, e))?;

        context.clear();
        context.func.signature = sig;
        let mut builder = FunctionBuilder::new(&mut context.func, builder_context);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // TODO: Implement actual method body compilation based on self.body
        if let Some(ret_type) = return_type {
            let zero = match ret_type {
                types::I32 | types::I64 => builder.ins().iconst(ret_type, 0),
                types::I8 => builder.ins().iconst(ret_type, 0),
                _ => {
                    return Err(format!(
                        "Unsupported return type for default value: {:?}\nIn method: {}.{}",
                        ret_type, current_class_name, self.name
                    ));
                }
            };
            builder.ins().return_(&[zero]);
        } else {
            builder.ins().return_(&[]); // Return void
        }

        builder.finalize();
        let flags = settings::Flags::new(settings::builder());
        verify_function(&context.func, &flags)
            .map_err(|e| format!("Function verification failed for {}: {}", func_name, e))?;
        module
            .define_function(func_id, context)
            .map_err(|e| format!("Failed to define function {}: {}", func_name, e))?;
        module.clear_context(context);
        Ok(())
    }
}

pub struct CodeGenerator {
    module: ObjectModule,
    context: Context,
    builder_context: FunctionBuilderContext,
    _data_context: DataContext,
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator {
    pub fn new() -> Self {
        // Use settings appropriate for the host by default; later configurable.
        let flag_builder = settings::builder();
        // flag_builder.enable("is_pic").unwrap(); // Position-independent code if needed

        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();

        let builder = ObjectBuilder::new(
            isa,
            "csharp_output", // The name of the output object file
            cranelift_module::default_libcall_names(),
        )
        .unwrap();
        let module = ObjectModule::new(builder);

        let context = module.make_context();
        let builder_context = FunctionBuilderContext::new();
        let data_context = DataContext::new();

        Self {
            module,
            context,
            builder_context,
            _data_context: data_context,
        }
    }

    pub fn compile(mut self, ast: &ast::CompilationUnit) -> Result<Vec<u8>, String> {
        // TODO: Implement compilation logic
        // For now, just process the AST structure and perhaps define functions

        // Handle global attributes (assembly and module metadata)
        if !ast.global_attributes.is_empty() {
            println!(
                "Processing {} global attributes",
                ast.global_attributes.len()
            );
            for global_attr in &ast.global_attributes {
                println!(
                    "Global attribute: {} -> {}",
                    global_attr.target.name, global_attr.attribute.name.name
                );
                // TODO: Implement metadata handling for assembly/module attributes
                // These typically affect metadata generation rather than code generation
            }
        }

        // Handle file-scoped namespace first if present
        if let Some(file_scoped_ns) = &ast.file_scoped_namespace {
            println!(
                "Compiling file-scoped namespace: {}",
                file_scoped_ns.name.name
            );
            for ns_member in &file_scoped_ns.declarations {
                if let NamespaceBodyDeclaration::Class(class_decl) = ns_member {
                    do_compile_class(
                        class_decl,
                        Some(file_scoped_ns.name.name.clone()),
                        &mut self.module,
                        &mut self.builder_context,
                        &mut self.context,
                    )?;
                }
            }
        }

        // Handle top-level declarations
        for declaration in &ast.declarations {
            match declaration {
                ast::TopLevelDeclaration::Namespace(ns) => {
                    // TODO: Handle namespace scoping in bytecode
                    for ns_member in &ns.declarations {
                        if let NamespaceBodyDeclaration::Class(class_decl) = ns_member {
                            do_compile_class(
                                class_decl,
                                Some(ns.name.name.clone()),
                                &mut self.module,
                                &mut self.builder_context,
                                &mut self.context,
                            )?;
                        }
                    }
                }
                ast::TopLevelDeclaration::FileScopedNamespace(file_scoped_ns) => {
                    println!(
                        "Compiling file-scoped namespace declaration: {}",
                        file_scoped_ns.name.name
                    );
                    for ns_member in &file_scoped_ns.declarations {
                        if let NamespaceBodyDeclaration::Class(class_decl) = ns_member {
                            do_compile_class(
                                class_decl,
                                Some(file_scoped_ns.name.name.clone()),
                                &mut self.module,
                                &mut self.builder_context,
                                &mut self.context,
                            )?;
                        }
                    }
                }
                ast::TopLevelDeclaration::Class(class_decl) => {
                    do_compile_class(
                        class_decl,
                        None,
                        &mut self.module,
                        &mut self.builder_context,
                        &mut self.context,
                    )?;
                }
                ast::TopLevelDeclaration::Struct(_struct_decl) => {
                    return Err("Struct compilation not implemented yet".to_string());
                }
                ast::TopLevelDeclaration::Record(_record_decl) => {
                    return Err("Record compilation not implemented yet".to_string());
                }
                ast::TopLevelDeclaration::Interface(_iface_decl) => {
                    return Err("Interface compilation not implemented yet".to_string());
                }
                ast::TopLevelDeclaration::Enum(_enum_decl) => {
                    return Err("Enum compilation not implemented yet".to_string());
                }
                ast::TopLevelDeclaration::Delegate(_delegate_decl) => {
                    return Err("Delegate compilation not implemented yet".to_string());
                }
                ast::TopLevelDeclaration::GlobalAttribute(_global_attr) => {
                    // Global attributes are already processed above
                    // They don't generate code directly, just metadata
                    continue;
                }
            }
        }

        // Handle top-level statements (C# 9+)
        if !ast.top_level_statements.is_empty() {
            println!(
                "Compiling {} top-level statements",
                ast.top_level_statements.len()
            );
            // TODO: Create a synthetic Main method for top-level statements
            // For now, just log that we found them
            for (i, _stmt) in ast.top_level_statements.iter().enumerate() {
                println!(
                    "Top-level statement {}: (compilation not yet implemented)",
                    i + 1
                );
            }
        }

        // Finalize the module and get the compiled object code
        match self.module.finish().emit() {
            Ok(obj_bytes) => Ok(obj_bytes),
            Err(e) => Err(format!("Failed to emit object code: {}", e)),
        }
    }
}

// Changed from a method to a free function
fn do_compile_class(
    class_decl: &ast::ClassDeclaration,
    parent_namespace: Option<String>,
    module: &mut ObjectModule,
    builder_context: &mut FunctionBuilderContext,
    context: &mut Context,
) -> Result<(), String> {
    // TODO: Implement actual class declaration visitation logic
    // This might involve calling class_decl.compile_node(...) if that's the pattern,
    // or other specific logic for classes.
    class_decl.compile_node(parent_namespace, module, builder_context, context)?;
    Ok(())
}

// TODO: Remove this stub once map_type is refactored or part of codegen context
fn map_type_stub(ty: &Type) -> Option<types::Type> {
    match ty {
        Type::Primitive(primitive) => match primitive {
            PrimitiveType::Void => None,
            PrimitiveType::Bool => Some(types::I8),

            // Integral types
            PrimitiveType::Byte => Some(types::I8),
            PrimitiveType::SByte => Some(types::I8),
            PrimitiveType::Short => Some(types::I16),
            PrimitiveType::UShort => Some(types::I16),
            PrimitiveType::Int => Some(types::I32),
            PrimitiveType::UInt => Some(types::I32),
            PrimitiveType::Long => Some(types::I64),
            PrimitiveType::ULong => Some(types::I64),
            // Pointer-sized ints map conservatively for now
            PrimitiveType::NInt => Some(types::I64),
            PrimitiveType::NUInt => Some(types::I64),

            // Floating-point types
            PrimitiveType::Float => Some(types::F32),
            PrimitiveType::Double => Some(types::F64),
            PrimitiveType::Decimal => {
                log::warn!(
                    "Decimal type mapping not fully implemented yet - using F64 as fallback."
                );
                Some(types::F64)
            }

            // Character and string types
            PrimitiveType::Char => Some(types::I16), // UTF-16 character
            PrimitiveType::String => {
                log::warn!("String type mapping not fully implemented yet.");
                None
            }
            PrimitiveType::Object => {
                log::warn!("Object type mapping not fully implemented yet.");
                None
            }
        },
        Type::Reference(name) => {
            log::warn!("Reference type mapping not implemented yet: {}", name);
            None
        }
        _ => None, // Handle other types (Generic, Array, Pointer, etc.)
    }
}

// Target triple is host by default via cranelift_native; make configurable in future.
