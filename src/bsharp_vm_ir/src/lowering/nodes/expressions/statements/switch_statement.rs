use bsharp_syntax::statements::SwitchStatement;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;

impl Lower<()> for SwitchStatement {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<(), CompileError> {
        Err(CompileError::new("E001", "Switch statement lowering not implemented"))
    }
}