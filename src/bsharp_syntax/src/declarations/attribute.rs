use crate::expressions::expression::Expression;
use crate::identifier::Identifier;
use crate::types::Type;
use bsharp_syntax_derive::AstNode;
use serde::{Deserialize, Serialize};

#[derive(AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: Identifier,
    pub arguments: Vec<Expression>,
    pub structured: Option<AttributeName>
}

#[derive(AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttributeList {
    pub attributes: Vec<Attribute>,
}

#[derive(AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttributeName {
    pub qualifier: Vec<Identifier>, // e.g., [System.Diagnostics.Conditional]
    pub name: Identifier,           // final segment
    pub type_arguments: Vec<Type>,  // generic attribute args on the final segment
}
