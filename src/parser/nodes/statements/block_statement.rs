use super::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}
