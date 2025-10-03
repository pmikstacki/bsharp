use super::SwitchSection;
use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchStatement {
    // The expression whose value is being switched on
    pub expression: Expression,
    // The sections within the switch block
    pub sections: Vec<SwitchSection>,
}
