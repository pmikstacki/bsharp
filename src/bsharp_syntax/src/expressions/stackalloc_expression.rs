use crate::expressions::Expression;
use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StackAllocExpression {
    pub target_type: Option<Type>,
    pub count: Option<Expression>,
    pub initializer: Option<Vec<Expression>>, // for stackalloc { ... }
}
