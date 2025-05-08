use serde::{Serialize, Deserialize};
use super::SwitchLabel;
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchSection<'a> {
    // Labels associated with this section (e.g., case 1: case 2:)
    pub labels: Vec<SwitchLabel<'a>>,
    // Statements within this section
    pub statements: Vec<Statement<'a>>,
}
