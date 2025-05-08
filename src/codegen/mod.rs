use crate::parser::ast;
use cranelift_codegen::ir::types;
use cranelift_codegen::ir::{AbiParam, InstBuilder};
use cranelift_codegen::settings::{self};
use cranelift_codegen::verifier::verify_function;
use cranelift_codegen::{Context};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use std::str::FromStr;
use target_lexicon::Triple;

use crate::parser::nodes::declarations::{ClassMember, MethodDeclaration, ClassDeclaration};
use crate::parser::nodes::types::{Type, PrimitiveType};

// Trait for AST nodes that can be compiled
pub trait Compilable {
    // TODO: Define the necessary context/parameters needed for compilation
    // For now, let's pass the main parts of the CodeGenerator mutably.
    fn compile_node(
        &self,
        class_name: Option<&str>, // Added context for class name
        module: &mut ObjectModule,
        builder_context: &mut FunctionBuilderContext,
        context: &mut Context,
        // data_context: &mut DataContext, // Add if needed for data later
        // symbol_table: &mut SymbolTable, // Add symbol table when needed
    ) -> Result<(), String>;
}

// Implement Compilable for ClassDeclaration
impl Compilable for ClassDeclaration<'_> {
    fn compile_node(
        &self,
        _parent_class_name: Option<&str>, // Currently unused for class
        module: &mut ObjectModule,
        builder_context: &mut FunctionBuilderContext,
        context: &mut Context,
    ) -> Result<(), String> {
        log::info!("Compiling class: {}", self.name);
        for member in &self.members {
            match member {
                ClassMember::Method(method_decl) => {
                    // Pass the current class name as context
                    method_decl.compile_node(Some(&self.name.name), module, builder_context, context)?;
                }
                ClassMember::Field(field_decl) => {
                    // TODO: Handle field declarations (e.g., allocate space in constructor?)
                    log::info!("Skipping field declaration (in trait impl): {}", field_decl.name);
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// Implement Compilable for MethodDeclaration
impl Compilable for MethodDeclaration<'_> {
    fn compile_node(
        &self,
        class_name: Option<&str>, // Receive class name context
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
            if let Some(param_type) = map_type_stub(&param.ty) { // Use stub for now
                sig.params.push(AbiParam::new(param_type));
            } else {
                return Err(format!("Unsupported parameter type: {:?}\nIn method: {}.{}", param.ty, current_class_name, self.name));
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
                 _ => return Err(format!("Unsupported return type for default value: {:?}\nIn method: {}.{}", ret_type, current_class_name, self.name)),
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

impl CodeGenerator {
    pub fn new() -> Self {
        // Use settings which are appropriate for the target triple.
        let _triple = Triple::from_str(TARGET_TRIPLE).unwrap();
        let flag_builder = settings::builder();
        // flag_builder.enable("is_pic").unwrap(); // Position-independent code if needed

        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder.finish(settings::Flags::new(flag_builder)).unwrap();

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

    pub fn compile(mut self, ast: &ast::SourceFile) -> Result<Vec<u8>, String> { 
        // TODO: Implement compilation logic
        // For now, just process the AST structure and perhaps define functions

        for member in &ast.members { 
            match member {
                ast::TopLevelMember::Class(class_decl) => {
                    // Call the compile_node method from the trait
                    // Pass None for class_name context at the top level
                    class_decl.compile_node(
                        None, 
                        &mut self.module, 
                        &mut self.builder_context, 
                        &mut self.context
                    )?;
                }
                ast::TopLevelMember::Namespace(namespace_decl) => {
                    // TODO: Handle namespace members recursively
                    eprintln!("Skipping namespace compilation: {}", namespace_decl.name);
                }
                // TODO: Handle other MemberDeclarationSyntax variants when added
            }
        }

        // Finalize the module and get the compiled object code
        match self.module.finish().emit() {
            Ok(obj_bytes) => Ok(obj_bytes),
            Err(e) => Err(format!("Failed to emit object code: {}", e)),
        }
    }
}

// TODO: Remove this stub once map_type is refactored or part of codegen context
fn map_type_stub(ty: &Type) -> Option<types::Type> {
    match ty {
        Type::Primitive(primitive) => match primitive {
            PrimitiveType::Void => None,
            PrimitiveType::Int => Some(types::I32),
            PrimitiveType::Bool => Some(types::I8),
            PrimitiveType::String => {
                log::warn!("String type mapping not fully implemented yet.");
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

// Define the target architecture
// TODO: Make this configurable
const TARGET_TRIPLE: &str = "x86_64-unknown-unknown-elf";
