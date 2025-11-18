use crate::builder::{IrFunctionBuilder, IrModuleBuilder};
use crate::instr::IrInstr;
use crate::module::{FunctionFlags, IrModule};

use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::lowering::context::LoweringContext;

impl Lower<IrModule> for bsharp_syntax::ast::CompilationUnit {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<IrModule, CompileError> {
        // Minimal lowering: create a script entry function with a single empty block that returns.
        let mut mod_builder = IrModuleBuilder::new();
        let flags = FunctionFlags { is_script_entry: true, is_intrinsic: false };
        let mut func_builder: IrFunctionBuilder = mod_builder.new_function("script_entry", flags);

        let entry_block = func_builder.new_block();

        // Set the active function in the lowering context so node lowerers can emit IR.
        ctx.set_active_function(func_builder, entry_block);

        // Dispatch top-level statements (currently stubs) before emitting the final return.
        for statement in &self.top_level_statements {
            // Best-effort lowering; errors propagate to caller for now.
            statement.lower(ctx)?;
        }

        // Take the function back, append a final return, and finalize it.
        let mut func_builder = ctx
            .take_active_function()
            .expect("active function must be present during CU lowering");
        func_builder
            .append_instr(entry_block, IrInstr::Return { value: None })
            .map_err(|e| CompileError::new("IR001", format!("Failed to append Return: {:?}", e)))?;

        let func = func_builder.build();
        let func_id = func.id;
        mod_builder.push_function(func);
        mod_builder.set_entry(func_id);

        let mut module = mod_builder.build();
        // Merge constants collected during expression lowering
        module.constants.extend(ctx.module.constants.drain(..));
        Ok(module)
    }
}

pub fn lower_compilation_unit(cu: &bsharp_syntax::ast::CompilationUnit) -> Result<IrModule, CompileError> {
    let mut ctx = LoweringContext::default();
    cu.lower(&mut ctx)
}
