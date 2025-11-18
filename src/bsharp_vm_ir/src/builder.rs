use std::collections::HashMap;

use crate::error::{IrError, IrResult};
use crate::ids::{BlockId, FunctionId, LocalId, RegisterId};
use crate::instr::IrInstr;
use crate::module::{FunctionFlags, IrBlock, IrFunction, IrLocal, IrModule, IrParam};
use crate::value::ValueKind;

pub struct IrModuleBuilder {
    module: IrModule,
    next_function_id: u32,
}

impl IrModuleBuilder {
    pub fn new() -> Self {
        Self {
            module: IrModule { functions: Vec::new(), globals: Vec::new(), constants: Vec::new(), entry: None },
            next_function_id: 0,
        }
    }

    pub fn set_entry(&mut self, func: FunctionId) {
        self.module.entry = Some(func);
    }

    pub fn new_function(&mut self, name: impl Into<String>, flags: FunctionFlags) -> IrFunctionBuilder {
        let id = FunctionId(self.next_function_id);
        self.next_function_id += 1;
        IrFunctionBuilder::new(id, name.into(), flags)
    }

    pub fn push_function(&mut self, func: IrFunction) {
        self.module.functions.push(func);
    }

    pub fn build(self) -> IrModule {
        self.module
    }
}

pub struct IrFunctionBuilder {
    func: IrFunction,
    next_block_id: u32,
    next_local_id: u32,
    next_register: u16,
    // Optional index for quick lookup; linear search is fine for now, but keep map if needed later
    _block_index: HashMap<u32, usize>,
}

impl IrFunctionBuilder {
    pub fn new(id: FunctionId, name: String, flags: FunctionFlags) -> Self {
        Self {
            func: IrFunction {
                id,
                name,
                params: Vec::new(),
                locals: Vec::new(),
                blocks: Vec::new(),
                register_count: 0,
                flags,
            },
            next_block_id: 0,
            next_local_id: 0,
            next_register: 0,
            _block_index: HashMap::new(),
        }
    }

    pub fn add_param(&mut self, name: Option<String>, kind: ValueKind) -> RegisterId {
        let reg = RegisterId(self.next_register);
        self.next_register = self.next_register.saturating_add(1);
        self.func.params.push(IrParam { name, kind, register: reg });
        reg
    }

    pub fn add_local(&mut self, name: Option<String>, kind: ValueKind) -> (LocalId, RegisterId) {
        let id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        let reg = RegisterId(self.next_register);
        self.next_register = self.next_register.saturating_add(1);
        self.func.locals.push(IrLocal { id, name, kind, register: reg });
        (id, reg)
    }

    pub fn new_register(&mut self) -> RegisterId {
        let reg = RegisterId(self.next_register);
        self.next_register = self.next_register.saturating_add(1);
        reg
    }

    pub fn new_block(&mut self) -> BlockId {
        let id = BlockId(self.next_block_id);
        self.next_block_id += 1;
        let idx = self.func.blocks.len();
        self.func.blocks.push(IrBlock { id, instructions: Vec::new() });
        self._block_index.insert(id.0, idx);
        id
    }

    pub fn append_instr(&mut self, block: BlockId, instr: IrInstr) -> IrResult<()> {
        if let Some(idx) = self._block_index.get(&block.0).copied() {
            if let Some(b) = self.func.blocks.get_mut(idx) {
                b.instructions.push(instr);
                return Ok(());
            }
        }
        // fallback linear search if map not in sync
        if let Some(b) = self.func.blocks.iter_mut().find(|b| b.id == block) {
            b.instructions.push(instr);
            return Ok(());
        }
        Err(IrError::InvalidBlockTarget(block))
    }

    pub fn build(mut self) -> IrFunction {
        self.func.register_count = self.next_register;
        self.func
    }
}
