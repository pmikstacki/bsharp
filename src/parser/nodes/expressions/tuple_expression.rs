use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TupleExpression<'a> {
    pub elements: Vec<TupleElement<'a>>,
}

use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TupleElement<'a> {
    pub name: Option<Identifier>, // None for unnamed
    pub value: Expression<'a>,
}
