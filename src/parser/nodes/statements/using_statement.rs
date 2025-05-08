use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UsingStatement<'a> {
    pub resource: Expression<'a>, // Could be a declaration or an expression
    pub body: Box<Statement<'a>>,
}
