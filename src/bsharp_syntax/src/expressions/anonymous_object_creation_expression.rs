use crate::expressions::Expression;
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnonymousObjectCreationExpression {
    pub initializers: Vec<AnonymousObjectMember>,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnonymousObjectMember {
    pub name: Option<Identifier>, // None for projection initializers
    pub value: Expression,
}
