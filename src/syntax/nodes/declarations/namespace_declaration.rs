use crate::syntax::nodes::declarations::{
    ClassDeclaration, DelegateDeclaration, EnumDeclaration, GlobalAttribute, InterfaceDeclaration,
    RecordDeclaration, StructDeclaration, UsingDirective,
};
use crate::syntax::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

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
}
