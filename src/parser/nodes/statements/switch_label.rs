use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression; // For case value

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SwitchLabel<'a> {
    Case(Expression<'a>), // case constant:
    Default,          // default:
}
