use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberAccessExpression<'a> {
    // The expression whose member is being accessed (e.g., Variable, another MemberAccess)
    pub object: Box<Expression<'a>>,
    // The name of the member being accessed
    pub member: Identifier,
}
