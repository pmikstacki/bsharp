use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NullConditionalExpression<'a> {
    pub target: Box<Expression<'a>>,
    pub member: Identifier, // or index for element access
    pub is_element_access: bool,
    pub argument: Option<Box<Expression<'a>>>, // for element/index access
}
