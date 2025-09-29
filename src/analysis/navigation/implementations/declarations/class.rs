use crate::syntax::nodes::{
    declarations::{ClassDeclaration, MethodDeclaration},
    expressions::expression::Expression,
    statements::statement::Statement,
};
use crate::analysis::navigation::traits::{AstNavigate, DeclarationInfo, DeclarationType, FindDeclarations};

impl AstNavigate for ClassDeclaration {
    fn find_if_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_if_statements());
        }
        results
    }

    fn find_for_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_for_loops());
        }
        results
    }

    fn find_while_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_while_loops());
        }
        results
    }

    fn find_switch_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_switch_statements());
        }
        results
    }

    fn find_try_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_try_statements());
        }
        results
    }

    fn find_using_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_using_statements());
        }
        results
    }

    fn find_expressions<F>(&self, predicate: F) -> Vec<&Expression>
    where
        F: Fn(&Expression) -> bool,
    {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_expressions(&predicate));
        }
        results
    }
}

impl FindDeclarations for ClassDeclaration {
    fn find_classes(&self) -> Vec<&ClassDeclaration> {
        vec![self]
    }

    fn find_methods(&self) -> Vec<&MethodDeclaration> {
        let mut methods = Vec::new();
        for member in &self.body_declarations {
            if let crate::syntax::nodes::declarations::ClassBodyDeclaration::Method(method) = member {
                methods.push(method);
            }
        }
        methods
    }

    fn find_interfaces(&self) -> Vec<&crate::syntax::nodes::declarations::InterfaceDeclaration> {
        Vec::new()
    }

    fn find_structs(&self) -> Vec<&crate::syntax::nodes::declarations::StructDeclaration> {
        Vec::new()
    }

    fn find_enums(&self) -> Vec<&crate::syntax::nodes::declarations::EnumDeclaration> {
        Vec::new()
    }

    fn find_records(&self) -> Vec<&crate::syntax::nodes::declarations::RecordDeclaration> {
        Vec::new()
    }

    fn find_delegates(&self) -> Vec<&crate::syntax::nodes::declarations::DelegateDeclaration> {
        Vec::new()
    }

    fn find_by_name(&self, name: &str) -> Vec<DeclarationInfo> {
        let mut results = Vec::new();

        if self.name.name == name {
            results.push(DeclarationInfo {
                name: self.name.name.clone(),
                declaration_type: DeclarationType::Class,
                location: None,
            });
        }

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
