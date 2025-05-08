use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum YieldStatement<'a> {
    Return(Expression<'a>),
    Break,
}
