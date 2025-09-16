use crate::syntax::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexingExpression {
    pub target: Box<Expression>,
    pub index: Box<Expression>,
}
