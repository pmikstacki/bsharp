use crate::syntax::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub consequence: Box<Expression>, // Expression if condition is true
    pub alternative: Box<Expression>, // Expression if condition is false
}
