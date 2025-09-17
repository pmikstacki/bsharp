// Added for initializer
use super::Modifier;
use crate::syntax::nodes::expressions::expression::Expression;
// Use absolute path for Type
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldDeclaration {
    pub modifiers: Vec<Modifier>, // Added modifiers support
    pub ty: Type,
    pub name: Identifier,
    pub initializer: Option<Expression>, // Added optional initializer
}
