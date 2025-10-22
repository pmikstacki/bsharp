use crate::Identifier;
use crate::declarations::{Attribute, Modifier};
use crate::statements::statement::Statement;
use bsharp_syntax_derive::AstNode;
use serde::{Deserialize, Serialize};

#[derive(AstNode, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DestructorDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub body: Option<Statement>,
}
