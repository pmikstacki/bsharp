use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Attribute<'a> {
    pub name: Identifier,
    pub arguments: Vec<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttributeList<'a> {
    pub attributes: Vec<Attribute<'a>>,
}
