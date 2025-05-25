use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UsingStatement {
    pub resource: Expression, // Could be a declaration or an expression
    pub body: Box<Statement>,
}
