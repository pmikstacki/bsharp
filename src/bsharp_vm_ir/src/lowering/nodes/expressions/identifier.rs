use bsharp_syntax::Identifier;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::{RegisterId, IrInstr};

impl Lower<RegisterId> for Identifier {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<RegisterId, CompileError> {
        let name = self.to_string();
        if let Some(src_reg) = ctx.get_local(&name) {
            // If needed, move into a fresh register to keep SSA-like semantics
            if let (Some(func), Some(block)) = (ctx.func.as_mut(), ctx.current_block) {
                let dst = func.new_register();
                func
                    .append_instr(block, IrInstr::Move { dst, src: src_reg })
                    .map_err(|e| CompileError::new("IR001", format!("Failed to append Move: {:?}", e)))?;
                return Ok(dst);
            }
            return Ok(src_reg);
        }
        Err(CompileError::new("E000", format!("Unbound identifier: {}", name)))
    }
}