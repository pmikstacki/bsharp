use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexingExpression<'a> {
    pub target: Box<Expression<'a>>,
    pub index: Box<Expression<'a>>,
}
