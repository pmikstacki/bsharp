use crate::analysis::navigation::traits::{AstNavigate, DeclarationInfo, DeclarationType, FindDeclarations};
use crate::syntax::nodes::{
    declarations::{DelegateDeclaration, ClassDeclaration, MethodDeclaration, InterfaceDeclaration, StructDeclaration, EnumDeclaration, RecordDeclaration},
    expressions::expression::Expression,
    statements::statement::Statement,
};

impl AstNavigate for DelegateDeclaration {
    fn find_if_statements(&self) -> Vec<&Statement> { Vec::new() }
    fn find_for_loops(&self) -> Vec<&Statement> { Vec::new() }
    fn find_while_loops(&self) -> Vec<&Statement> { Vec::new() }
    fn find_switch_statements(&self) -> Vec<&Statement> { Vec::new() }
    fn find_try_statements(&self) -> Vec<&Statement> { Vec::new() }
    fn find_using_statements(&self) -> Vec<&Statement> { Vec::new() }
    fn find_expressions<F>(&self, _predicate: F) -> Vec<&Expression>
    where F: Fn(&Expression) -> bool { Vec::new() }
}

impl FindDeclarations for DelegateDeclaration {
    fn find_classes(&self) -> Vec<&ClassDeclaration> { Vec::new() }
    fn find_methods(&self) -> Vec<&MethodDeclaration> { Vec::new() }
    fn find_interfaces(&self) -> Vec<&InterfaceDeclaration> { Vec::new() }
    fn find_structs(&self) -> Vec<&StructDeclaration> { Vec::new() }
    fn find_enums(&self) -> Vec<&EnumDeclaration> { Vec::new() }
    fn find_records(&self) -> Vec<&RecordDeclaration> { Vec::new() }
    fn find_delegates(&self) -> Vec<&DelegateDeclaration> { vec![self] }
    fn find_by_name(&self, name: &str) -> Vec<DeclarationInfo> {
        if self.name.name == name {
            vec![DeclarationInfo { name: self.name.name.clone(), declaration_type: DeclarationType::Delegate, location: None }]
        } else { Vec::new() }
    }
}
