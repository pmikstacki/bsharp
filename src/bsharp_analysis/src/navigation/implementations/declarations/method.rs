use crate::navigation::{DeclarationInfo, DeclarationType};
use crate::syntax::nodes::{
    declarations::{
        ClassDeclaration, DelegateDeclaration, EnumDeclaration, InterfaceDeclaration,
        MethodDeclaration, RecordDeclaration, StructDeclaration,
    },
    expressions::expression::Expression,
    statements::statement::Statement,
};
use crate::{AstNavigate, FindDeclarations};

impl AstNavigate for MethodDeclaration {
    fn find_if_statements(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_if_statements()
        } else {
            Vec::new()
        }
    }

    fn find_for_loops(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_for_loops()
        } else {
            Vec::new()
        }
    }

    fn find_while_loops(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_while_loops()
        } else {
            Vec::new()
        }
    }

    fn find_switch_statements(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_switch_statements()
        } else {
            Vec::new()
        }
    }

    fn find_try_statements(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_try_statements()
        } else {
            Vec::new()
        }
    }

    fn find_using_statements(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_using_statements()
        } else {
            Vec::new()
        }
    }

    fn find_expressions<F>(&self, predicate: F) -> Vec<&Expression>
    where
        F: Fn(&Expression) -> bool,
    {
        if let Some(body) = &self.body {
            body.find_expressions(predicate)
        } else {
            Vec::new()
        }
    }
}

impl FindDeclarations for MethodDeclaration {
    fn find_classes(&self) -> Vec<&ClassDeclaration> {
        Vec::new()
    }

    fn find_methods(&self) -> Vec<&MethodDeclaration> {
        vec![self]
    }

    fn find_interfaces(&self) -> Vec<&InterfaceDeclaration> {
        Vec::new()
    }

    fn find_structs(&self) -> Vec<&StructDeclaration> {
        Vec::new()
    }

    fn find_enums(&self) -> Vec<&EnumDeclaration> {
        Vec::new()
    }

    fn find_records(&self) -> Vec<&RecordDeclaration> {
        Vec::new()
    }

    fn find_delegates(&self) -> Vec<&DelegateDeclaration> {
        Vec::new()
    }

    fn find_by_name(&self, name: &str) -> Vec<DeclarationInfo> {
        if self.name.name == name {
            vec![DeclarationInfo {
                name: self.name.name.clone(),
                declaration_type: DeclarationType::Method,
                location: None,
            }]
        } else {
            Vec::new()
        }
    }
}
