use crate::ids::{BlockId, FunctionId, GlobalId, LocalId, RegisterId};
use crate::instr::IrInstr;
use crate::value::ValueKind;

#[derive(Debug, Clone, Default)]
pub struct IrModule {
    pub functions: Vec<IrFunction>,
    pub globals: Vec<IrGlobal>,
    pub constants: Vec<IrConstant>,
    pub entry: Option<FunctionId>,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub id: FunctionId,
    pub name: String,
    pub params: Vec<IrParam>,
    pub locals: Vec<IrLocal>,
    pub blocks: Vec<IrBlock>,
    pub register_count: u16,
    pub flags: FunctionFlags,
}

#[derive(Debug, Clone)]
pub struct IrBlock {
    pub id: BlockId,
    pub instructions: Vec<IrInstr>,
}

#[derive(Debug, Clone)]
pub struct IrParam {
    pub name: Option<String>,
    pub kind: ValueKind,
    pub register: RegisterId,
}

#[derive(Debug, Clone)]
pub struct IrLocal {
    pub id: LocalId,
    pub name: Option<String>,
    pub kind: ValueKind,
    pub register: RegisterId,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FunctionFlags {
    pub is_script_entry: bool,
    pub is_intrinsic: bool,
}

#[derive(Debug, Clone)]
pub struct IrGlobal {
    pub id: GlobalId,
    pub name: String,
    pub kind: ValueKind,
    pub initial_value: Option<IrConstant>,
}

#[derive(Debug, Clone)]
pub enum IrConstant {
    Int32(i32),
    Bool(bool),
    String(String),
    Null,
}
