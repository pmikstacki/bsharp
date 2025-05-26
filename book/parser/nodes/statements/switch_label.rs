use crate::parser::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};
// For case value

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SwitchLabel {
    Case(Expression), // case constant:
    Default,          // default:
}
