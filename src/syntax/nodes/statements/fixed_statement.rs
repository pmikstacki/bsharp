use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedStatement {
    pub var_type: Type,
    pub var_name: Identifier,
    pub initializer: Expression,
    pub body: Box<Statement>,
}
