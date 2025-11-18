use crate::ids::{BlockId, FunctionId, RegisterId};

#[derive(Debug)]
pub enum IrError {
    InvalidBlockTarget(BlockId),
    InvalidRegister(RegisterId),
    MissingTerminator(BlockId),
    InvalidFunction(FunctionId),
}

pub type IrResult<T> = Result<T, IrError>;
