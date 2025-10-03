use crate::Identifier;
use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ArgumentModifier {
    Ref,
    Out,
    In,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub name: Option<Identifier>,
    pub modifier: Option<ArgumentModifier>,
    pub expr: Expression,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvocationExpression {
    // The expression being called (e.g., Variable, MemberAccess)
    pub callee: Box<Expression>,
    // The arguments passed to the method
    pub arguments: Vec<Argument>,
}
