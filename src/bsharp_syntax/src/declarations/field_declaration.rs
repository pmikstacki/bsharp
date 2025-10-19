// Added for initializer
use super::Modifier;
use crate::expressions::Expression;
use crate::types::Type;
use crate::Identifier;
use bsharp_syntax_derive::AstNode;
use serde::{Deserialize, Serialize};

#[derive(AstNode, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldDeclaration {
    pub modifiers: Vec<Modifier>, // Added modifiers support
    pub field_type: Type,
    pub name: Identifier,
    pub initializer: Option<Expression>, // Added optional initializer
}
