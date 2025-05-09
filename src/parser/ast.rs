use serde::{Serialize, Deserialize};
// Make these re-exports public so other modules (like codegen) can use ast::TypeName
pub use super::nodes::declarations::{
    UsingDirective, NamespaceDeclaration, ClassDeclaration, StructDeclaration, RecordDeclaration,
    InterfaceDeclaration, EnumDeclaration, // Add DelegateDeclaration if it's also needed publicly via ast
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompilationUnit<'a> {
    pub usings: Vec<UsingDirective>,
    pub members: Vec<TopLevelMember<'a>>, // Can be Namespace or Class/Struct/etc.
}

// Ensure TopLevelMember also derives PartialEq for CompilationUnit's derive to work.
// It should also have other common derives for AST nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)] 
pub enum TopLevelMember<'a> {
    Namespace(NamespaceDeclaration<'a>),
    Class(ClassDeclaration<'a>),
    Struct(StructDeclaration<'a>),
    Record(RecordDeclaration<'a>),
    Interface(InterfaceDeclaration<'a>),
    Enum(EnumDeclaration<'a>),
    // TODO: Add other top-level members like delegates, global attributes later
}
