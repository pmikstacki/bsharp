// Make these re-exports public so other modules (like codegen) can use ast::TypeName
pub use super::nodes::declarations::{
    ClassDeclaration, DelegateDeclaration, EnumDeclaration, InterfaceDeclaration, NamespaceDeclaration,
    RecordDeclaration, StructDeclaration, UsingDirective, FileScopedNamespaceDeclaration, GlobalAttribute, // Added GlobalAttribute
};
use crate::parser::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    pub global_attributes: Vec<GlobalAttribute>, // Assembly and module attributes
    pub using_directives: Vec<UsingDirective>,
    pub declarations: Vec<TopLevelDeclaration>, // Can be Namespace or Class/Struct/etc.
    pub file_scoped_namespace: Option<FileScopedNamespaceDeclaration>, // C# 10+ file-scoped namespace
    pub top_level_statements: Vec<Statement>, // C# 9+ top-level statements
}

// Ensure TopLevelDeclaration also derives PartialEq for CompilationUnit's derive to work.
// It should also have other common derives for AST nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)] 
pub enum TopLevelDeclaration {
    Namespace(NamespaceDeclaration),
    FileScopedNamespace(FileScopedNamespaceDeclaration), // C# 10+ file-scoped namespace
    Class(ClassDeclaration),
    Struct(StructDeclaration),
    Record(RecordDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Delegate(DelegateDeclaration), // Added Delegate variant
    GlobalAttribute(GlobalAttribute), // Global assembly/module attributes
}
