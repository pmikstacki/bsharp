use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnonymousObjectCreationExpression<'a> {
    pub initializers: Vec<AnonymousObjectMember<'a>>,
}

use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnonymousObjectMember<'a> {
    pub name: Option<Identifier>, // None for projection initializers
    pub value: Expression<'a>,
}
