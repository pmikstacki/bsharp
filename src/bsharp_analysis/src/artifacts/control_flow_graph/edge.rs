use serde::{Deserialize, Serialize};

/// Kinds of control-flow edges between blocks.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum EdgeKind {
    Normal,
    True,
    False,
    SwitchCase,
    Exception,
    Finally,
}
