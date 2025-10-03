use crate::Identifier;
use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberAccessExpression {
    // The expression whose member is being accessed (e.g., Variable, another MemberAccess)
    pub object: Box<Expression>,
    // The name of the member being accessed
    pub member: Identifier,
}
