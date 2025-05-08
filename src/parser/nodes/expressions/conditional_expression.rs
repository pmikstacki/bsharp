use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConditionalExpression<'a> {
    pub condition: Box<Expression<'a>>,
    pub consequence: Box<Expression<'a>>, // ExpressionSyntax if condition is true
    pub alternative: Box<Expression<'a>>, // ExpressionSyntax if condition is false
}
