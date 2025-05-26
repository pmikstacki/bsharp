use crate::parser::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameofExpression {
    pub expr: Box<Expression>,
}
