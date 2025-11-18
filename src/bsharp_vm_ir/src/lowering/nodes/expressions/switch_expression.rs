use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::RegisterId;

impl Lower<RegisterId> for bsharp_syntax::expressions::expression::SwitchExpression {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<RegisterId, CompileError> {
        Err(CompileError::new("E000", "Switch expression lowering not implemented"))
    }
}