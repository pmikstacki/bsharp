use crate::parser::nodes::types::Type;
use serde::{Deserialize, Serialize};
use crate::parser::nodes::expressions::expression::Expression;
// Use absolute path for Type
use crate::parser::nodes::identifier::Identifier;
// Added for initializer
use super::Modifier;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldDeclaration {
    pub modifiers: Vec<Modifier>, // Added modifiers support
    pub ty: Type,
    pub name: Identifier,
    pub initializer: Option<Expression>, // Added optional initializer
}
