use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TupleExpression {
    pub elements: Vec<TupleElement>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TupleElement {
    pub name: Option<Identifier>, // None for unnamed
    pub value: Expression,
}
