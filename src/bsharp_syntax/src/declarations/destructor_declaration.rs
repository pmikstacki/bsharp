use crate::declarations::{Attribute, Modifier};
use crate::statements::statement::Statement;
use crate::Identifier;
use serde::{Deserialize, Serialize};
use bsharp_syntax_derive::AstNode;

#[derive(AstNode, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DestructorDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub body: Option<Statement>,
}
