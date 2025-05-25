use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LockStatement {
    pub expr: Expression,
    pub body: Box<Statement>,
}
