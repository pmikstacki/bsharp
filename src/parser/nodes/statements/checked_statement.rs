use serde::{Serialize, Deserialize};
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CheckedStatement {
    pub body: Box<Statement>, // Usually Statement::Block
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UncheckedStatement {
    pub body: Box<Statement>,
}
