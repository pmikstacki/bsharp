use crate::parser::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvocationExpression {
    // The expression being called (e.g., Variable, MemberAccess)
    pub callee: Box<Expression>,
    // The arguments passed to the method
    pub arguments: Vec<Expression>,
}
