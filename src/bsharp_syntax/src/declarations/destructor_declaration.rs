use crate::Identifier;
use crate::declarations::{Attribute, Modifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DestructorDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub body: String, // body or signature
}
