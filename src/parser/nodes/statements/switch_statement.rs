use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use super::SwitchSection;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchStatement<'a> {
    // The expression whose value is being switched on
    pub expression: Expression<'a>,
    // The sections within the switch block
    pub sections: Vec<SwitchSection<'a>>,
}
