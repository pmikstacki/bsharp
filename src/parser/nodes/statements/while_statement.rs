use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: Box<Expression>,
    // While loop body is typically a block, but can be a single statement.
    pub body: Box<Statement>, // Expecting Statement::Block usually
}
