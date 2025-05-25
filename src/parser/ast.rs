// Make these re-exports public so other modules (like codegen) can use ast::TypeName
pub use super::nodes::declarations::{
    ClassDeclaration, DelegateDeclaration, EnumDeclaration, InterfaceDeclaration, NamespaceDeclaration,
    RecordDeclaration, StructDeclaration, UsingDirective, // Added DelegateDeclaration
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    pub using_directives: Vec<UsingDirective>,
    pub declarations: Vec<TopLevelDeclaration>, // Can be Namespace or Class/Struct/etc.
}

// Ensure TopLevelDeclaration also derives PartialEq for CompilationUnit's derive to work.
// It should also have other common derives for AST nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)] 
pub enum TopLevelDeclaration {
    Namespace(NamespaceDeclaration),
    Class(ClassDeclaration),
    Struct(StructDeclaration),
    Record(RecordDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Delegate(DelegateDeclaration), // Added Delegate variant
    // TODO: Add other top-level members like global attributes later
}
