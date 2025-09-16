use crate::parser::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnsafeStatement {
    pub body: Box<Statement>,
}
