use bsharp_syntax::expressions::StackAllocExpression;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::RegisterId;

impl Lower<RegisterId> for StackAllocExpression {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<RegisterId, CompileError> {
        Err(CompileError::new("E000", "Stackalloc expression lowering not implemented"))
    }
}