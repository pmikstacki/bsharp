use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GotoCaseKind {
    Case(Expression),
    Default,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GotoCaseStatement {
    pub kind: GotoCaseKind,
}
