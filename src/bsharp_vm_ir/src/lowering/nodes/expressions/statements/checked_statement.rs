use bsharp_syntax::statements::CheckedStatement;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;

// Complex statement stubs
impl Lower<()> for CheckedStatement {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<(), CompileError> {
        Err(CompileError::new("E001", "Checked statement lowering not implemented"))
    }
}