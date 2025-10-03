use crate::expressions::Expression;
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
