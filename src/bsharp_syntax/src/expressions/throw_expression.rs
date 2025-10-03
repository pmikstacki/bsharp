use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ThrowExpression {
    pub expr: Option<Box<Expression>>, // None for 'throw;' in expressions
}
