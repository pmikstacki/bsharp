use crate::syntax::ast::CompilationUnit;
use crate::syntax::nodes::{
    declarations::{
        ClassDeclaration, DelegateDeclaration, EnumDeclaration, InterfaceDeclaration,
        MethodDeclaration, RecordDeclaration, StructDeclaration,
    },
    expressions::expression::Expression,
    statements::statement::Statement,
};
use crate::analysis::navigation::traits::{AstNavigate, DeclarationInfo, DeclarationType, FindDeclarations};

// Internal helpers to streamline iteration over declarations across namespaces and file-scoped namespaces
fn iter_namespace_members(
    cu: &CompilationUnit,
) -> impl Iterator<
    Item = &crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration,
> + '_ {
    let top_level = cu
        .declarations
        .iter()
        .filter_map(|member| {
            if let crate::syntax::ast::TopLevelDeclaration::Namespace(ns) = member {
                Some(ns.declarations.iter())
            } else {
                None
            }
        })
        .flatten();

    let file_scoped = cu
        .file_scoped_namespace
        .iter()
        .flat_map(|fs| fs.declarations.iter());

    top_level.chain(file_scoped)
}

impl AstNavigate for CompilationUnit {
    fn find_if_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_if_statements());
        }
        results
    }

    fn find_for_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_for_loops());
        }
        results
    }

    fn find_while_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_while_loops());
        }
        results
    }

    fn find_switch_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_switch_statements());
        }
        results
    }

    fn find_try_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_try_statements());
        }
        results
    }

    fn find_using_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_using_statements());
        }
        results
    }

    fn find_expressions<F>(&self, predicate: F) -> Vec<&Expression>
    where
        F: Fn(&Expression) -> bool,
    {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_expressions(&predicate));
        }
        results
    }
}

impl FindDeclarations for CompilationUnit {
    fn find_classes(&self) -> Vec<&ClassDeclaration> {
        let mut classes = Vec::new();

        // Direct top-level classes
        for member in &self.declarations {
            if let crate::syntax::ast::TopLevelDeclaration::Class(class) = member {
                classes.push(class);
            }
        }

        // Classes inside namespaces (including file-scoped)
        for ns_member in iter_namespace_members(self) {
            if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(class) = ns_member {
                classes.push(class);
            }
        }

        classes
    }

    fn find_methods(&self) -> Vec<&MethodDeclaration> {
        let mut methods = Vec::new();
        for class in self.find_classes() {
            methods.extend(class.find_methods());
        }
        methods
    }

    fn find_interfaces(&self) -> Vec<&InterfaceDeclaration> {
        let mut interfaces = Vec::new();

        // Search top-level declarations
        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Interface(interface) = ns_member {
                            interfaces.push(interface);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Interface(interface) => {
                    interfaces.push(interface);
                }
                _ => {}
            }
        }

        // Search file-scoped namespace
        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Interface(interface) = declaration {
                    interfaces.push(interface);
                }
            }
        }

        interfaces
    }

    fn find_structs(&self) -> Vec<&StructDeclaration> {
        let mut structs = Vec::new();

        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Struct(struct_decl) = ns_member {
                            structs.push(struct_decl);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Struct(struct_decl) => {
                    structs.push(struct_decl);
                }
                _ => {}
            }
        }

        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Struct(struct_decl) = declaration {
                    structs.push(struct_decl);
                }
            }
        }

        structs
    }

    fn find_enums(&self) -> Vec<&EnumDeclaration> {
        let mut enums = Vec::new();

        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Enum(enum_decl) = ns_member {
                            enums.push(enum_decl);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Enum(enum_decl) => {
                    enums.push(enum_decl);
                }
                _ => {}
            }
        }

        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Enum(enum_decl) = declaration {
                    enums.push(enum_decl);
                }
            }
        }

        enums
    }

    fn find_records(&self) -> Vec<&RecordDeclaration> {
        let mut records = Vec::new();

        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Record(record_decl) = ns_member {
                            records.push(record_decl);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Record(record_decl) => {
                    records.push(record_decl);
                }
                _ => {}
            }
        }

        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Record(record_decl) = declaration {
                    records.push(record_decl);
                }
            }
        }

        records
    }

    fn find_delegates(&self) -> Vec<&DelegateDeclaration> {
        let mut delegates = Vec::new();

        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Delegate(delegate_decl) = ns_member {
                            delegates.push(delegate_decl);
                        }
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Delegate(delegate_decl) => {
                    delegates.push(delegate_decl);
                }
                _ => {}
            }
        }

        if let Some(_file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &self.file_scoped_namespace.as_ref().unwrap().declarations {
                if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Delegate(delegate_decl) = declaration {
                    delegates.push(delegate_decl);
                }
            }
        }

        delegates
    }

    fn find_by_name(&self, name: &str) -> Vec<DeclarationInfo> {
        let mut results = Vec::new();

        // Find classes
        for class in self.find_classes() {
            if class.name.name == name {
                results.push(DeclarationInfo {
                    name: class.name.name.clone(),
                    declaration_type: DeclarationType::Class,
                    location: None,
                });
            }
        }

        // Find interfaces
        for interface in self.find_interfaces() {
            if interface.name.name == name {
                results.push(DeclarationInfo {
                    name: interface.name.name.clone(),
                    declaration_type: DeclarationType::Interface,
                    location: None,
                });
            }
        }

        // Find methods
        for method in self.find_methods() {
            if method.name.name == name {
                results.push(DeclarationInfo {
                    name: method.name.name.clone(),
                    declaration_type: DeclarationType::Method,
                    location: None,
                });
            }
        }

        results
    }
}
