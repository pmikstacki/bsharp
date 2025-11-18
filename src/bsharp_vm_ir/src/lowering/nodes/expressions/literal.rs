use bsharp_syntax::expressions::Literal;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::{RegisterId, IrConstant};
use crate::instr::IrInstr;

// Literal expressions
impl Lower<RegisterId> for Literal {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<RegisterId, CompileError> {
        let block = ctx.current_block()?;
        let dst: RegisterId = ctx.new_register()?;

        // Map literal to IR constant and optionally value kind (kept for future type tracking)
        let const_index = match self {
            Literal::Integer(v) => {
                // Narrow to i32 for now; real impl may select Int64/others
                ctx.push_constant(IrConstant::Int32(*v as i32))
            }
            Literal::IntegerWithSuffix(v, _suf) => {
                ctx.push_constant(IrConstant::Int32(*v as i32))
            }
            Literal::Boolean(b) => {
                ctx.push_constant(IrConstant::Bool(*b))
            }
            Literal::String(s) | Literal::VerbatimString(s) | Literal::RawString(s) => {
                ctx.push_constant(IrConstant::String(s.clone()))
            }
            Literal::Null => {
                ctx.push_constant(IrConstant::Null)
            }
            Literal::Float(f) => {
                // No float IR constant yet; fallback via string for now
                ctx.push_constant(IrConstant::String(f.to_string()))
            }
            Literal::Decimal(s) => {
                ctx.push_constant(IrConstant::String(s.clone()))
            }
            Literal::Utf8String(bytes) => {
                let s = String::from_utf8_lossy(bytes).to_string();
                ctx.push_constant(IrConstant::String(s))
            }
            Literal::Char(c) => {
                ctx.push_constant(IrConstant::String(c.to_string()))
            }
            Literal::InterpolatedString(_isp) => {
                return Err(CompileError::new("E000", "Interpolated string literal lowering not implemented"));
            }
        };

        ctx.emit(block, IrInstr::LoadConst { dst, const_index })?;

        Ok(dst)
    }
}