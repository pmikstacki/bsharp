use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub consequence: Box<Expression>, // Expression if condition is true
    pub alternative: Box<Expression>, // Expression if condition is false
}
