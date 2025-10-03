use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ArrayIndexExpression {
    // The expression producing the array/collection being indexed
    pub array: Box<Expression>,
    // The expression used as the index
    pub index: Box<Expression>,
}
