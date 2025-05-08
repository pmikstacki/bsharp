use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::declarations::{
    ClassDeclaration, StructDeclaration, InterfaceDeclaration, EnumDeclaration, DelegateDeclaration, RecordDeclaration, GlobalAttribute, UsingDirective
};
use crate::parser::nodes::preprocessor::PreprocessorDirective;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamespaceDeclaration<'a> {
    pub name: Identifier, 
    // TODO: Add support for nested namespaces later
    pub usings: Vec<UsingDirective>,
    // Members can be nested namespaces or type declarations
    pub members: Vec<NamespaceMember<'a>>, 
}

// Define what can be a member of a namespace
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NamespaceMember<'a> {
    Namespace(NamespaceDeclaration<'a>),
    Class(ClassDeclaration<'a>),
    Struct(StructDeclaration<'a>),
    Interface(InterfaceDeclaration<'a>),
    Enum(EnumDeclaration<'a>),
    Delegate(DelegateDeclaration<'a>),
    Record(RecordDeclaration<'a>),
    GlobalAttribute(GlobalAttribute<'a>),
    Preprocessor(PreprocessorDirective), 
}
