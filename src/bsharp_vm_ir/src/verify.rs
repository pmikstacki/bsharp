use crate::error::{IrError, IrResult};
use crate::instr::IrInstr;
use crate::module::{IrBlock, IrFunction, IrModule};

pub struct IrVerifier;

impl IrVerifier {
    pub fn verify_module(module: &IrModule) -> IrResult<()> {
        for f in &module.functions {
            Self::verify_function(f)?;
        }
        Ok(())
    }

    pub fn verify_function(func: &IrFunction) -> IrResult<()> {
        // For now: ensure each block ends with a terminator and has at least one instruction.
        for b in &func.blocks {
            Self::verify_block(b)?;
        }
        Ok(())
    }

    fn verify_block(block: &IrBlock) -> IrResult<()> {
        if block.instructions.is_empty() {
            return Err(IrError::MissingTerminator(block.id));
        }
        let last = block.instructions.last().expect("checked non-empty");
        if !is_terminator(last) {
            return Err(IrError::MissingTerminator(block.id));
        }
        Ok(())
    }
}

fn is_terminator(i: &IrInstr) -> bool {
    match i {
        IrInstr::Return { .. } => true,
        IrInstr::Jump { .. } => true,
        IrInstr::JumpIfTrue { .. } => true,
        IrInstr::JumpIfFalse { .. } => true,
        _ => false,
    }
}
