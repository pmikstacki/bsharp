use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedStatement {
    pub var_type: Type,
    pub var_name: Identifier,
    pub initializer: Expression,
    pub body: Box<Statement>,
}
