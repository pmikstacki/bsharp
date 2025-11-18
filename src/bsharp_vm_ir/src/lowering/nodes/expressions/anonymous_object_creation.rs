use bsharp_syntax::expressions::AnonymousObjectCreationExpression;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::RegisterId;

// Complex expression stubs
impl Lower<RegisterId> for AnonymousObjectCreationExpression {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<RegisterId, CompileError> {
        Err(CompileError::new("E000", "Anonymous object creation expression lowering not implemented"))
    }
}