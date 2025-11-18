use bsharp_syntax::statements::LabelStatement;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;

impl Lower<()> for LabelStatement {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<(), CompileError> {
        Err(CompileError::new("E001", "Label statement lowering not implemented"))
    }
}