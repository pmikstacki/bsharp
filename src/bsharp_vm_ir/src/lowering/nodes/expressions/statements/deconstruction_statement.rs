use bsharp_syntax::expressions::DeconstructionExpression;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;

impl Lower<()> for DeconstructionExpression {
        fn lower(&self, _ctx: &mut LoweringContext) -> Result<(), CompileError> {
            Err(CompileError::new("E001", "Deconstruction expression lowering not implemented"))
        }
    }