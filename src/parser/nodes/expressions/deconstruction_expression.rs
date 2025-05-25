use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeconstructionExpression {
    pub variables: Vec<Identifier>,
    pub value: Box<Expression>,
}
