use crate::expressions::Expression;
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TupleExpression {
    pub elements: Vec<TupleElement>,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TupleElement {
    pub name: Option<Identifier>, // None for unnamed
    pub value: Expression,
}
