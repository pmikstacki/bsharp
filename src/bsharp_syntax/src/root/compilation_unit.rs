use crate::ast::TopLevelDeclaration;
use crate::declarations::{FileScopedNamespaceDeclaration, GlobalAttribute, UsingDirective};
use crate::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    pub global_attributes: Vec<GlobalAttribute>, // Assembly and module attributes
    pub using_directives: Vec<UsingDirective>,
    pub declarations: Vec<TopLevelDeclaration>, // Can be Namespace or Class/Struct/etc.
    pub file_scoped_namespace: Option<FileScopedNamespaceDeclaration>, // C# 10+ file-scoped namespace
    pub top_level_statements: Vec<Statement>,                          // C# 9+ top-level statements
}
