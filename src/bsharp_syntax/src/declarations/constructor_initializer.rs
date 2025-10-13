use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConstructorInitializer {
    Base(Vec<Expression>),
    This(Vec<Expression>),
}
