use serde::{Serialize, Deserialize};
use crate::parser::nodes::declarations::{Attribute, Modifier};
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DestructorDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub body: String, // body or signature
}
