use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameofExpression<'a> {
    pub expr: Box<Expression<'a>>,
}
