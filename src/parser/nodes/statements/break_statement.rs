use serde::{Serialize, Deserialize};

// Represents a 'break;' statement.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct BreakStatement;
