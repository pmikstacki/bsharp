use bsharp_syntax::declarations::LocalVariableDeclaration;
use bsharp_syntax::declarations::local_variable_declaration::VariableDeclaration;
use bsharp_syntax::statements::LocalFunctionStatement;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::{ValueKind, IrInstr};

impl Lower<()> for LocalVariableDeclaration {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<(), CompileError> {
        let block = match ctx.current_block {
            Some(b) => b,
            None => return Err(CompileError::new("E001", "No active block for local declaration lowering")),
        };

        // Very basic kind mapping for now; real typing TBD
        let kind = ValueKind::ObjectRef;

        for VariableDeclaration { name, initializer } in &self.declarators {
            let name_str = name.to_string();

            let dst_reg = {
                let func = ctx
                    .func
                    .as_mut()
                    .ok_or_else(|| CompileError::new("E001", "No active function for local declaration lowering"))?;
                let (_local_id, dst) = func.add_local(Some(name_str.clone()), kind);
                dst
            };

            ctx.bind_local(name_str.clone(), dst_reg);

            if let Some(init_expr) = initializer {
                let src_reg: crate::RegisterId = init_expr.lower(ctx)?;
                let func = ctx
                    .func
                    .as_mut()
                    .ok_or_else(|| CompileError::new("E001", "No active function for local init lowering"))?;
                func
                    .append_instr(block, IrInstr::Move { dst: dst_reg, src: src_reg })
                    .map_err(|e| CompileError::new("IR001", format!("Failed to append Move: {:?}", e)))?;
            }
        }

        Ok(())
    }
}

impl Lower<()> for LocalFunctionStatement {
    fn lower(&self, _ctx: &mut LoweringContext) -> Result<(), CompileError> {
        Err(CompileError::new("E001", "Local function statement lowering not implemented"))
    }
}