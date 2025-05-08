use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LockStatement<'a> {
    pub expr: Expression<'a>,
    pub body: Box<Statement<'a>>,
}
