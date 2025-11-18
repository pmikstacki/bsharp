use bsharp_vm_ir::FunctionId;

use crate::program::BytecodeProgram;
use crate::types::{CallFrame, HeapObject, Value};

#[derive(Debug)]
pub struct Vm {
    pub program: BytecodeProgram,
    pub registers: Vec<Value>,
    pub call_stack: Vec<CallFrame>,
    pub heap: Vec<Option<HeapObject>>,
    pub free_list: Vec<u32>,
}

#[derive(Debug)]
pub enum VmError {
    TypeMismatch,
    DivideByZero,
    InvalidOpcode(u8),
    InvalidFunction(FunctionId),
    InvalidRegister(u16),
    NotImplemented,
}

pub type VmResult<T> = Result<T, VmError>;

impl Vm {
    pub fn new(program: BytecodeProgram) -> Vm {
        Vm {
            program,
            registers: Vec::new(),
            call_stack: Vec::new(),
            heap: Vec::new(),
            free_list: Vec::new(),
        }
    }

    pub fn run(&mut self) -> VmResult<Value> {
        Err(VmError::NotImplemented)
    }
}
