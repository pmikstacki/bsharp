use super::SwitchLabel;
use crate::parser::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchSection {
    // Labels associated with this section (e.g., case 1: case 2:)
    pub labels: Vec<SwitchLabel>,
    // Statements within this section
    pub statements: Vec<Statement>,
}
