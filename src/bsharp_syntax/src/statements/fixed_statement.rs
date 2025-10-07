use crate::expressions::Expression;
use crate::statements::statement::Statement;
use crate::types::Type;
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedStatement {
    pub var_type: Type,
    pub var_name: Identifier,
    pub initializer: Expression,
    pub body: Box<Statement>,
}
