use crate::syntax::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GotoCaseKind {
    Case(Expression),
    Default,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GotoCaseStatement {
    pub kind: GotoCaseKind,
}
