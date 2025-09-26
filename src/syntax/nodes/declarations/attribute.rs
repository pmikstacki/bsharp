use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: Identifier,
    pub arguments: Vec<Expression>,
    pub structured: Option<AttributeName>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttributeList {
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttributeName {
    pub qualifier: Vec<Identifier>, // e.g., [System.Diagnostics.Conditional]
    pub name: Identifier,           // final segment
    pub type_arguments: Vec<Type>,  // generic attribute args on the final segment
}
