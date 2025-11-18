use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;

pub trait Lower<T> {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<T, CompileError>;
}
