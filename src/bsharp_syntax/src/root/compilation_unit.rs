use crate::ast::TopLevelDeclaration;
use crate::declarations::{
    FileScopedNamespaceDeclaration, GlobalAttribute, GlobalUsingDirective, UsingDirective,
};
use crate::statements::statement::Statement;
use serde::{Deserialize, Serialize};
use bsharp_syntax_derive::AstNode;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, AstNode)]
pub struct CompilationUnit {
    pub global_attributes: Vec<GlobalAttribute>,
    pub using_directives: Vec<UsingDirective>,
    pub global_using_directives: Vec<GlobalUsingDirective>,
    pub declarations: Vec<TopLevelDeclaration>,
    pub file_scoped_namespace: Option<FileScopedNamespaceDeclaration>,
    pub top_level_statements: Vec<Statement>,
}
