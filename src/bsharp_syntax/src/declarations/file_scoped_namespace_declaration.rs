use crate::declarations::{NamespaceBodyDeclaration, UsingDirective};
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileScopedNamespaceDeclaration {
    pub name: Identifier, // The namespace name (e.g., MyOrg.MyApp)
    pub declarations: Vec<NamespaceBodyDeclaration>, // Declarations within this file-scoped namespace
    pub using_directives: Vec<UsingDirective>,
}
