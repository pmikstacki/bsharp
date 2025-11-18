use bsharp_syntax::statements::BreakStatement;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;

// Simple statement stubs
impl Lower<()> for BreakStatement {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<(), CompileError> {
        Err(CompileError::new("E001", "Break statement lowering not implemented"))
    }
}