use crate::expressions::Expression;
use crate::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: Box<Expression>,
    // While loop body is typically a block, but can be a single statement.
    pub body: Box<Statement>, // Expecting Statement::Block usually
}
