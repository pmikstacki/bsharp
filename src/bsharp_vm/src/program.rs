use bsharp_vm_ir::FunctionId;

use crate::types::BytecodeWord;

#[derive(Debug, Clone)]
pub struct BytecodeFunction {
    pub id: FunctionId,
    pub code: Vec<BytecodeWord>,
    pub register_count: u16,
}

#[derive(Debug, Clone)]
pub struct BytecodeProgram {
    pub functions: Vec<BytecodeFunction>,
    pub entry: FunctionId,
}
