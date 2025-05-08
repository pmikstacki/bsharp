use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WhileStatement<'a> {
    pub condition: Box<Expression<'a>>,
    // While loop body is typically a block, but can be a single statement.
    pub body: Box<Statement<'a>>, // Expecting Statement::Block usually
}
