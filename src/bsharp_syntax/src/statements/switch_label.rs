use crate::expressions::{Expression, Pattern};
use serde::{Deserialize, Serialize};
// For case value

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SwitchLabel {
    Case(Expression), // case constant:
    Default,          // default:
    Pattern {
        // case <pattern> [when <expr>]:
        pattern: Pattern,
        when_clause: Option<Expression>,
    },
}
