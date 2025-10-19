use crate::declarations::{
    ClassDeclaration, DelegateDeclaration, EnumDeclaration, FileScopedNamespaceDeclaration,
    GlobalAttribute, InterfaceDeclaration, NamespaceDeclaration, RecordDeclaration,
    StructDeclaration,
};
use serde::{Deserialize, Serialize};
use bsharp_syntax_derive::AstNode;

// Ensure TopLevelDeclaration also derives PartialEq for CompilationUnit's derive to work.
// It should also have other common derives for AST nodes.
#[derive(AstNode, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopLevelDeclaration {
    Namespace(NamespaceDeclaration),
    FileScopedNamespace(FileScopedNamespaceDeclaration), // C# 10+ file-scoped namespace
    Class(ClassDeclaration),
    Struct(StructDeclaration),
    Record(RecordDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Delegate(DelegateDeclaration),    // Added Delegate variant
    GlobalAttribute(GlobalAttribute), // Global assembly/module attributes
}
