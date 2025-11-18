use crate::ids::{BlockId, FunctionId, RegisterId};

#[derive(Debug, Clone)]
pub enum CapturedVar {
    Local(RegisterId),
    Upvalue(u16),
}

#[derive(Debug, Clone)]
pub enum IrInstr {
    Nop,
    LoadConst { dst: RegisterId, const_index: u16 },
    Move { dst: RegisterId, src: RegisterId },
    AddInt { dst: RegisterId, lhs: RegisterId, rhs: RegisterId },
    SubInt { dst: RegisterId, lhs: RegisterId, rhs: RegisterId },
    NegInt { dst: RegisterId, src: RegisterId },
    NotBool { dst: RegisterId, src: RegisterId },
    Return { value: Option<RegisterId> },
    Call {
        dst: Option<RegisterId>,
        function: FunctionId,
        args: Vec<RegisterId>,
    },
    CallIntrinsic {
        dst: Option<RegisterId>,
        intrinsic_id: u16,
        args: Vec<RegisterId>,
    },
    CallDynamic {
        dst: Option<RegisterId>,
        callee: RegisterId,
        args: Vec<RegisterId>,
    },
    MakeClosure {
        dst: RegisterId,
        function: FunctionId,
        captured: Vec<CapturedVar>,
    },
    LoadUpvalue {
        dst: RegisterId,
        upvalue_index: u16,
    },
    StoreUpvalue {
        upvalue_index: u16,
        src: RegisterId,
    },
    Jump {
        target: BlockId,
    },
    JumpIfTrue {
        cond: RegisterId,
        target: BlockId,
    },
    JumpIfFalse {
        cond: RegisterId,
        target: BlockId,
    },
}
