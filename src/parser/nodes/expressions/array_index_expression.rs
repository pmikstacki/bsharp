use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ArrayIndexExpression<'a> {
    // The expression producing the array/collection being indexed
    pub array: Box<Expression<'a>>,
    // The expression used as the index
    pub index: Box<Expression<'a>>,
}
