use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ThrowExpression<'a> {
    pub expr: Option<Box<Expression<'a>>>, // None for 'throw;' in expressions
}
