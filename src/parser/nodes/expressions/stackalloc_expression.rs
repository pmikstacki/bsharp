use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StackAllocExpression {
    pub ty: Option<Type>,
    pub count: Option<Expression>,
    pub initializer: Option<Vec<Expression>>, // for stackalloc { ... }
}
