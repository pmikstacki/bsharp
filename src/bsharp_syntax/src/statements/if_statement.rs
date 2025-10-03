use crate::expressions::Expression;
use crate::statements::statement::Statement;
use serde::{Deserialize, Serialize};
// Use StatementSyntax from the same directory's mod.rs

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    // An if statement's body is typically a block, but can be a single statement.
    // We'll enforce Block for simplicity now, can refine later based on grammar.
    pub consequence: Box<Statement>,
    // Optional else block or else-if statement
    pub alternative: Option<Box<Statement>>,
}
