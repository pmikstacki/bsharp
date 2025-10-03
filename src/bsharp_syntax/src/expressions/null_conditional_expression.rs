use crate::Identifier;
use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NullConditionalExpression {
    pub target: Box<Expression>,
    pub member: Identifier, // or index for element access
    pub is_element_access: bool,
    pub argument: Option<Box<Expression>>, // for element/index access
}
