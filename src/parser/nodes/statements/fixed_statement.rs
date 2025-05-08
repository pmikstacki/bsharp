use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedStatement<'a> {
    pub var_type: Type<'a>,
    pub var_name: Identifier,
    pub initializer: Expression<'a>,
    pub body: Box<Statement<'a>>,
}
