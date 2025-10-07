use crate::artifacts::control_flow_graph::terminator::Terminator;
use serde::{Deserialize, Serialize};

/// Identifier of a basic block within a CFG.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BlockId(pub u32);

/// A basic block is a straight-line sequence of statements ending with a terminator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub id: BlockId,
    pub statements: Vec<u32>, // optional indices/handles to statements
    pub terminator: Terminator,
}