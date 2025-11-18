use bsharp_vm_ir::FunctionId;

pub type BytecodeWord = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectHandle {
    pub index: u32,
    pub generation: u16,
}

#[derive(Debug, Clone)]
pub enum HeapObject {
    Closure(ClosureObject),
}

#[derive(Debug, Clone)]
pub struct ClosureObject {
    pub function: FunctionId,
    pub upvalues: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Int32(i32),
    Bool(bool),
    String(String),
    Null,
    Object(ObjectHandle),
    Function(FunctionId),
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function: FunctionId,
    pub ip: usize,
    pub base: usize,
}
