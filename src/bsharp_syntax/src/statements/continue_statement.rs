use bsharp_syntax_derive::AstNode;
use serde::{Deserialize, Serialize};

// Represents a 'continue;' statement.
#[derive(AstNode, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct ContinueStatement;
