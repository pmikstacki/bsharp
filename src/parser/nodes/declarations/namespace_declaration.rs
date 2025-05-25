use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::declarations::{
    ClassDeclaration, StructDeclaration, InterfaceDeclaration, EnumDeclaration, DelegateDeclaration, RecordDeclaration, GlobalAttribute, UsingDirective
};
use crate::parser::nodes::preprocessor::PreprocessorDirective;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamespaceDeclaration {
    pub name: Identifier, 
    // TODO: Add support for nested namespaces later
    pub using_directives: Vec<UsingDirective>,
    // Members can be nested namespaces or type declarations
    pub declarations: Vec<NamespaceBodyDeclaration>, 
}

// Define what can be a member of a namespace
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NamespaceBodyDeclaration {
    Namespace(NamespaceDeclaration),
    Class(ClassDeclaration),
    Struct(StructDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Delegate(DelegateDeclaration),
    Record(RecordDeclaration),
    GlobalAttribute(GlobalAttribute),
    Preprocessor(PreprocessorDirective), 
}
