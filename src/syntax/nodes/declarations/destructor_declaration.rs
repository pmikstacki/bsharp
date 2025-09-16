use crate::syntax::nodes::declarations::{Attribute, Modifier};
use crate::syntax::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DestructorDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub body: String, // body or signature
}
