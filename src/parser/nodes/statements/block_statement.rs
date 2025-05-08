use serde::{Serialize, Deserialize};
use super::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}
