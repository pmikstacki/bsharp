use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexingExpression {
    pub target: Box<Expression>,
    pub index: Box<Expression>,
}
