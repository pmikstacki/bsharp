use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StackAllocExpression<'a> {
    pub ty: Option<Type<'a>>,
    pub count: Option<Expression<'a>>,
    pub initializer: Option<Vec<Expression<'a>>>, // for stackalloc { ... }
}
