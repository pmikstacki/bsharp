use serde::{Deserialize, Serialize};

// Represents a 'continue;' statement.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct ContinueStatement;
