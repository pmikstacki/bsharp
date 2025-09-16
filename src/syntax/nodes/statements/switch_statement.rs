use super::SwitchSection;
use crate::syntax::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchStatement {
    // The expression whose value is being switched on
    pub expression: Expression,
    // The sections within the switch block
    pub sections: Vec<SwitchSection>,
}
