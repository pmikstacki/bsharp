use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GotoCaseKind<'a> {
    Case(Expression<'a>),
    Default,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GotoCaseStatement<'a> {
    pub kind: GotoCaseKind<'a>,
}
