use serde::{Deserialize, Serialize};
use bsharp_syntax_derive::AstNode;

// Represents a 'continue;' statement.
#[derive(AstNode, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct ContinueStatement;
