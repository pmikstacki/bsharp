use crate::module::{IrModule, IrConstant};
use crate::builder::IrFunctionBuilder;
use crate::ids::{BlockId, RegisterId};
use crate::instr::IrInstr;
use crate::lowering::error::CompileError;
use std::collections::HashMap;

#[derive(Default)]
pub struct LoweringContext {
    pub module: IrModule,
    pub func: Option<IrFunctionBuilder>,
    pub current_block: Option<BlockId>,
    pub locals: HashMap<String, crate::RegisterId>,
    bool_const_indices: [Option<u16>; 2],
}

impl LoweringContext {
    pub fn set_active_function(&mut self, func: IrFunctionBuilder, block: BlockId) {
        self.func = Some(func);
        self.current_block = Some(block);
    }

    pub fn take_active_function(&mut self) -> Option<IrFunctionBuilder> {
        self.current_block = None;
        self.func.take()
    }

    pub fn bind_local(&mut self, name: String, reg: crate::RegisterId) {
        self.locals.insert(name, reg);
    }

    pub fn get_local(&self, name: &str) -> Option<crate::RegisterId> {
        self.locals.get(name).copied()
    }

    pub fn ensure_function(&mut self) -> Result<&mut IrFunctionBuilder, CompileError> {
        self.func
            .as_mut()
            .ok_or_else(|| CompileError::new("E001", "No active function available for lowering"))
    }

    pub fn current_block(&self) -> Result<BlockId, CompileError> {
        self.current_block
            .ok_or_else(|| CompileError::new("E001", "No active block available for lowering"))
    }

    pub fn new_block(&mut self) -> Result<BlockId, CompileError> {
        Ok(self.ensure_function()?.new_block())
    }

    pub fn new_register(&mut self) -> Result<RegisterId, CompileError> {
        Ok(self.ensure_function()?.new_register())
    }

    pub fn emit(&mut self, block: BlockId, instr: IrInstr) -> Result<(), CompileError> {
        self.ensure_function()?
            .append_instr(block, instr)
            .map_err(|e| CompileError::new("IR001", format!("Failed to append instruction: {:?}", e)))
    }

    pub fn load_bool(&mut self, block: BlockId, dst: RegisterId, value: bool) -> Result<(), CompileError> {
        let const_index = self.bool_const_index(value);
        self.emit(block, IrInstr::LoadConst { dst, const_index })
    }

    pub fn push_constant(&mut self, constant: IrConstant) -> u16 {
        self.module.constants.push(constant);
        (self.module.constants.len() - 1) as u16
    }

    fn bool_const_index(&mut self, value: bool) -> u16 {
        let slot = if value { 1 } else { 0 };
        if let Some(idx) = self.bool_const_indices[slot] {
            return idx;
        }

        let idx = self.push_constant(IrConstant::Bool(value));
        self.bool_const_indices[slot] = Some(idx);
        idx
    }
}
