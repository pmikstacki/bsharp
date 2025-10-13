use crate::artifacts::control_flow_graph::block::BlockId;
use serde::{Deserialize, Serialize};

/// Block terminators that define outgoing control flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum Terminator {
    Goto(BlockId),
    If {
        then_bb: BlockId,
        else_bb: BlockId,
    },
    Switch {
        targets: Vec<(i32, BlockId)>,
        default: Option<BlockId>,
    },
    Return,
    Throw,
    #[default]
    Unreachable,
}

