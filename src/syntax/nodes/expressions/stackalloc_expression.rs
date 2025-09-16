use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StackAllocExpression {
    pub ty: Option<Type>,
    pub count: Option<Expression>,
    pub initializer: Option<Vec<Expression>>, // for stackalloc { ... }
}
