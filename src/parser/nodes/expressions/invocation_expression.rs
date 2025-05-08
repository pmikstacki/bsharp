use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvocationExpression<'a> {
    // The expression being called (e.g., Variable, MemberAccess)
    pub callee: Box<Expression<'a>>,
    // The arguments passed to the method
    pub arguments: Vec<Expression<'a>>,
}
