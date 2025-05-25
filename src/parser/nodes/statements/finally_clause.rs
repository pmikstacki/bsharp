use serde::{Serialize, Deserialize};
use crate::parser::nodes::statements::statement::Statement; // For the block

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinallyClause {
    // The block of statements to execute regardless of exceptions
    pub block: Box<Statement>, // Must be Statement::Block
}
