use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexingExpression {
    pub target: Box<Expression>,
    pub index: Box<Expression>,
}
