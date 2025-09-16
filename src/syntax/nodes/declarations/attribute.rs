use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: Identifier,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttributeList {
    pub attributes: Vec<Attribute>,
}
