use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StackAllocExpression {
    pub ty: Option<Type>,
    pub count: Option<Expression>,
    pub initializer: Option<Vec<Expression>>, // for stackalloc { ... }
}
