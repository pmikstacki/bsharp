use serde::{Serialize, Deserialize};
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnsafeStatement {
    pub body: Box<Statement>,
}
