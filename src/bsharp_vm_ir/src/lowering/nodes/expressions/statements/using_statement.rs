use bsharp_syntax::statements::UsingStatement;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;

impl Lower<()> for UsingStatement {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<(), CompileError> {
        Err(CompileError::new("E001", "Using statement lowering not implemented"))
    }
}