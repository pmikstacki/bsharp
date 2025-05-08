use serde::{Serialize, Deserialize};
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CheckedStatement<'a> {
    pub body: Box<Statement<'a>>, // Usually Statement::Block
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UncheckedStatement<'a> {
    pub body: Box<Statement<'a>>,
}
