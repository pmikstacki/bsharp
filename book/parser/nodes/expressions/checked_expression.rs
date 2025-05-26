use crate::parser::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CheckedExpression {
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UncheckedExpression {
    pub expr: Box<Expression>,
}
