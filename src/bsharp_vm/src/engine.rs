use crate::program::BytecodeProgram;
use crate::types::Value;
use crate::vm::{Vm, VmResult};

pub struct ScriptEngine;

impl ScriptEngine {
    pub fn new() -> ScriptEngine {
        ScriptEngine
    }

    pub fn run_program(&self, program: BytecodeProgram) -> VmResult<Value> {
        let mut vm = Vm::new(program);
        vm.run()
    }
}
