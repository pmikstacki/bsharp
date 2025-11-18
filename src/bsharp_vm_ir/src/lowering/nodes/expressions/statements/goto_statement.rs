use bsharp_syntax::statements::{GotoCaseStatement, GotoStatement};
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;

impl Lower<()> for GotoStatement {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<(), CompileError> {
        Err(CompileError::new("E001", "Goto statement lowering not implemented"))
    }
}

impl Lower<()> for GotoCaseStatement {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<(), CompileError> {
        Err(CompileError::new("E001", "Goto case statement lowering not implemented"))
    }
}