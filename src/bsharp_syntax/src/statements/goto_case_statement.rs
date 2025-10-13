use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GotoCaseKind {
    Case(Expression),
    Default,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GotoCaseStatement {
    pub kind: GotoCaseKind,
}
