use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use bsharp_syntax::statements::{
    BreakStatement, CheckedStatement, ContinueStatement, DoWhileStatement, FixedStatement,
    ForEachStatement, ForStatement, GotoCaseStatement, GotoStatement, IfStatement,
    LabelStatement, LocalFunctionStatement, LockStatement, SwitchStatement, TryStatement,
    UncheckedStatement, UnsafeStatement, UsingStatement, WhileStatement, YieldStatement
};
use bsharp_syntax::declarations::LocalVariableDeclaration;
use bsharp_syntax::expressions::DeconstructionExpression;

impl Lower<()> for bsharp_syntax::statements::statement::Statement {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<(), CompileError> {
        match self {
            bsharp_syntax::statements::statement::Statement::Goto(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::GotoCase(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Label(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Checked(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Unchecked(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Lock(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Using(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Yield(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Unsafe(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Fixed(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Try(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::ForEach(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Switch(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::DoWhile(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Break(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Continue(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::For(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::While(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::If(statement) => statement.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Declaration(declaration) => declaration.lower(ctx),
            bsharp_syntax::statements::statement::Statement::LocalFunction(local_function) => local_function.lower(ctx),
            bsharp_syntax::statements::statement::Statement::Expression(expression) => {
                // Expression statements evaluate the expression and discard the result
                expression.lower(ctx).map(|_| ())
            }
            bsharp_syntax::statements::statement::Statement::Return(expression) => {
                let block = ctx.current_block()?;
                match expression {
                    Some(expr) => {
                        let ret_reg = expr.lower(ctx)?;
                        ctx.emit(block, crate::instr::IrInstr::Return { value: Some(ret_reg) })?;
                        Ok(())
                    }
                    None => {
                        ctx.emit(block, crate::instr::IrInstr::Return { value: None })?;
                        Ok(())
                    }
                }
            }
            bsharp_syntax::statements::statement::Statement::Throw(expression) => {
                match expression {
                    Some(expr) => {
                        let _register = expr.lower(ctx)?;
                        Err(CompileError::new("E001", "Throw statement lowering not implemented"))
                    }
                    None => Err(CompileError::new("E001", "Throw statement lowering not implemented"))
                }
            }
            bsharp_syntax::statements::statement::Statement::Block(statements) => {
                for statement in statements {
                    statement.lower(ctx)?;
                }
                Ok(())
            }
            bsharp_syntax::statements::statement::Statement::Empty => Ok(()),
            bsharp_syntax::statements::statement::Statement::Deconstruction(expression) => {
                let _unit: () = expression.lower(ctx)?;
                Err(CompileError::new("E001", "Deconstruction statement lowering not implemented"))
            }
        }
    }
}

