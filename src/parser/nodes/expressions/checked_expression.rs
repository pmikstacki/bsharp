use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CheckedExpression<'a> {
    pub expr: Box<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UncheckedExpression<'a> {
    pub expr: Box<Expression<'a>>,
}
