use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type; // Use absolute path for Type
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::expressions::expression::Expression; // Added for initializer

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldDeclaration<'a> {
    // TODO: Add modifiers (public, static, readonly, etc.)
    pub ty: Type<'a>,
    pub name: Identifier,
    pub initializer: Option<Expression<'a>>, // Added optional initializer
}
