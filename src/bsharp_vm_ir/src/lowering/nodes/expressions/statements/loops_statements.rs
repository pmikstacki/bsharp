use bsharp_syntax::statements::{DoWhileStatement, ForStatement, WhileStatement};
use bsharp_syntax::statements::for_statement::ForInitializer;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::instr::IrInstr;

impl Lower<()> for DoWhileStatement {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<(), CompileError> {
        // Create body and cond/end blocks
        let body_block = ctx.new_block()?;
        let cond_block = ctx.new_block()?;
        let end_block = ctx.new_block()?;

        // Jump to body first
        let cur = ctx.current_block()?;
        ctx.emit(cur, IrInstr::Jump { target: body_block })?;

        // Lower body then jump to condition
        ctx.current_block = Some(body_block);
        self.body.lower(ctx)?;
        ctx.emit(body_block, IrInstr::Jump { target: cond_block })?;

        // Evaluate condition and branch back to body or exit
        ctx.current_block = Some(cond_block);
        let cond_reg = self.condition.lower(ctx)?;
        ctx.emit(cond_block, IrInstr::JumpIfTrue { cond: cond_reg, target: body_block })?;
        ctx.emit(cond_block, IrInstr::Jump { target: end_block })?;

        ctx.current_block = Some(end_block);
        Ok(())
    }
}

impl Lower<()> for ForStatement {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<(), CompileError> {
        // Handle initializer
        if let Some(init) = &self.initializer {
            match init {
                ForInitializer::Declaration(decl) => decl.lower(ctx)?,
                ForInitializer::Expressions(exprs) => {
                    for e in exprs { let _ = e.lower(ctx)?; }
                }
            }
        }

        // Create cond/body/iter/end blocks
        let cond_block = ctx.new_block()?;
        let body_block = ctx.new_block()?;
        let iter_block = ctx.new_block()?;
        let end_block = ctx.new_block()?;

        // Jump to condition first
        let cur = ctx.current_block()?;
        ctx.emit(cur, IrInstr::Jump { target: cond_block })?;

        // Condition check
        ctx.current_block = Some(cond_block);
        if let Some(cond) = &self.condition {
            let cond_reg = cond.lower(ctx)?;
            ctx.emit(cond_block, IrInstr::JumpIfFalse { cond: cond_reg, target: end_block })?;
        }

        // Body
        ctx.emit(cond_block, IrInstr::Jump { target: body_block })?;
        ctx.current_block = Some(body_block);
        self.body.lower(ctx)?;
        ctx.emit(body_block, IrInstr::Jump { target: iter_block })?;

        // Iterators
        ctx.current_block = Some(iter_block);
        for it in &self.iterator { let _ = it.lower(ctx)?; }
        ctx.emit(iter_block, IrInstr::Jump { target: cond_block })?;

        // After loop
        ctx.current_block = Some(end_block);
        Ok(())
    }
}

impl Lower<()> for WhileStatement {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<(), CompileError> {
        let cond_block = ctx.new_block()?;
        let body_block = ctx.new_block()?;
        let end_block = ctx.new_block()?;

        // Jump to condition
        let cur = ctx.current_block()?;
        ctx.emit(cur, IrInstr::Jump { target: cond_block })?;

        // Condition
        ctx.current_block = Some(cond_block);
        let cond_reg = self.condition.lower(ctx)?;
        ctx.emit(cond_block, IrInstr::JumpIfFalse { cond: cond_reg, target: end_block })?;
        ctx.emit(cond_block, IrInstr::Jump { target: body_block })?;

        // Body
        ctx.current_block = Some(body_block);
        self.body.lower(ctx)?;
        ctx.emit(body_block, IrInstr::Jump { target: cond_block })?;

        // End
        ctx.current_block = Some(end_block);
        Ok(())
    }
}