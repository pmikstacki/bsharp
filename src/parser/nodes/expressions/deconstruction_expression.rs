use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeconstructionExpression {
    pub variables: Vec<Identifier>,
    pub value: Box<Expression>,
}
