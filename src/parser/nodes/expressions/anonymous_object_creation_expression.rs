use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnonymousObjectCreationExpression {
    pub initializers: Vec<AnonymousObjectMember>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnonymousObjectMember {
    pub name: Option<Identifier>, // None for projection initializers
    pub value: Expression,
}
