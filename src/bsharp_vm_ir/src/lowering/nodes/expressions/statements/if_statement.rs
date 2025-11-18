use bsharp_syntax::statements::IfStatement;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::instr::IrInstr;

impl Lower<()> for IfStatement {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<(), CompileError> {
        // Evaluate condition
        let cond_reg = self.condition.lower(ctx)?;

        // Prepare blocks
        let then_block = ctx.new_block()?;
        let else_block = ctx.new_block()?;
        let end_block = ctx.new_block()?;

        // Emit branch on condition from current block
        let cur_block = ctx.current_block()?;
        ctx.emit(cur_block, IrInstr::JumpIfFalse { cond: cond_reg, target: else_block })?;

        // Lower then branch
        ctx.current_block = Some(then_block);
        self.consequence.lower(ctx)?;
        ctx.emit(then_block, IrInstr::Jump { target: end_block })?;

        // Lower else branch (if any)
        ctx.current_block = Some(else_block);
        if let Some(alternative) = &self.alternative {
            alternative.lower(ctx)?;
        }
        ctx.emit(else_block, IrInstr::Jump { target: end_block })?;

        // Continue in end block
        ctx.current_block = Some(end_block);
        Ok(())
    }
}