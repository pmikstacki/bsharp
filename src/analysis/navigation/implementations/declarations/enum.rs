use crate::analysis::navigation::traits::{AstNavigate, DeclarationInfo, DeclarationType, FindDeclarations};
use crate::syntax::nodes::{
    declarations::{EnumDeclaration, ClassDeclaration, MethodDeclaration, InterfaceDeclaration, StructDeclaration, RecordDeclaration, DelegateDeclaration},
    expressions::expression::Expression,
    statements::statement::Statement,
};

impl AstNavigate for EnumDeclaration {
    fn find_if_statements(&self) -> Vec<&Statement> { Vec::new() }
    fn find_for_loops(&self) -> Vec<&Statement> { Vec::new() }
    fn find_while_loops(&self) -> Vec<&Statement> { Vec::new() }
    fn find_switch_statements(&self) -> Vec<&Statement> { Vec::new() }
    fn find_try_statements(&self) -> Vec<&Statement> { Vec::new() }
    fn find_using_statements(&self) -> Vec<&Statement> { Vec::new() }
    fn find_expressions<F>(&self, _predicate: F) -> Vec<&Expression>
    where F: Fn(&Expression) -> bool { Vec::new() }
}

impl FindDeclarations for EnumDeclaration {
    fn find_classes(&self) -> Vec<&ClassDeclaration> { Vec::new() }
    fn find_methods(&self) -> Vec<&MethodDeclaration> { Vec::new() }
    fn find_interfaces(&self) -> Vec<&InterfaceDeclaration> { Vec::new() }
    fn find_structs(&self) -> Vec<&StructDeclaration> { Vec::new() }
    fn find_enums(&self) -> Vec<&EnumDeclaration> { vec![self] }
    fn find_records(&self) -> Vec<&RecordDeclaration> { Vec::new() }
    fn find_delegates(&self) -> Vec<&DelegateDeclaration> { Vec::new() }
    fn find_by_name(&self, name: &str) -> Vec<DeclarationInfo> {
        if self.name.name == name {
            vec![DeclarationInfo { name: self.name.name.clone(), declaration_type: DeclarationType::Enum, location: None }]
        } else { Vec::new() }
    }
}
