use crate::ids::RegisterId;
use crate::lowering::context::LoweringContext;
use crate::lowering::error::CompileError;
use crate::lowering::traits::Lower;
use crate::instr::IrInstr;
use bsharp_syntax::expressions::{
    AnonymousMethodExpression, AnonymousObjectCreationExpression, AssignmentExpression, AwaitExpression,
    CheckedExpression, ConditionalExpression, DeconstructionExpression, DefaultExpression,
    IndexExpression, InvocationExpression, LambdaExpression,
    Literal, MemberAccessExpression, NameofExpression, NewExpression,
    NullConditionalExpression, Pattern, QueryExpression,
    RangeExpression, SizeofExpression, StackAllocExpression, ThrowExpression,
    TupleExpression, TypeofExpression, UncheckedExpression
};
// trimmed unused imports

impl Lower<RegisterId> for bsharp_syntax::expressions::expression::Expression {
    fn lower(&self, ctx: &mut LoweringContext) -> Result<RegisterId, CompileError> {
        match self {
            bsharp_syntax::expressions::expression::Expression::AnonymousObject(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Tuple(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Range(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Index(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Pattern(pattern) => pattern.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Deconstruction(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Conditional(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::New(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::MemberAccess(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::NullConditional(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Invocation(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Assignment(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Literal(literal) => literal.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Variable(identifier) => identifier.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Unary { op, expr } => {
                use bsharp_syntax::expressions::unary_operator::UnaryOperator;
                let operand = expr.lower(ctx)?;
                match op {
                    UnaryOperator::Minus => {
                        let block = ctx.current_block()?;
                        let dst = ctx.new_register()?;
                        ctx.emit(block, IrInstr::NegInt { dst, src: operand })?;
                        Ok(dst)
                    }
                    UnaryOperator::LogicalNot => {
                        let block = ctx.current_block()?;
                        let dst = ctx.new_register()?;
                        ctx.emit(block, IrInstr::NotBool { dst, src: operand })?;
                        Ok(dst)
                    }
                    _ => Err(CompileError::new("E000", format!("Unary operator not implemented: {}", op))),
                }
            }
            bsharp_syntax::expressions::expression::Expression::Binary { left, op, right } => {
                use bsharp_syntax::expressions::binary_operator::BinaryOperator;
                let left_reg = left.lower(ctx)?;
                let right_reg = right.lower(ctx)?;
                match op {
                    BinaryOperator::Add => {
                        let block = ctx.current_block()?;
                        let dst = ctx.new_register()?;
                        ctx.emit(block, IrInstr::AddInt { dst, lhs: left_reg, rhs: right_reg })?;
                        Ok(dst)
                    }
                    BinaryOperator::Subtract => {
                        let block = ctx.current_block()?;
                        let dst = ctx.new_register()?;
                        ctx.emit(block, IrInstr::SubInt { dst, lhs: left_reg, rhs: right_reg })?;
                        Ok(dst)
                    }
                    BinaryOperator::LogicalAnd => {
                        // Short-circuit: if left is false => result false, else evaluate right
                        let then_block = ctx.new_block()?;
                        let end_block = ctx.new_block()?;
                        let dst = ctx.new_register()?;
                        let cur_block = ctx.current_block()?;

                        ctx.load_bool(cur_block, dst, false)?;
                        ctx.emit(cur_block, IrInstr::JumpIfFalse { cond: left_reg, target: end_block })?;
                        ctx.emit(cur_block, IrInstr::Jump { target: then_block })?;

                        ctx.current_block = Some(then_block);
                        let rhs_reg = right.lower(ctx)?;
                        ctx.emit(then_block, IrInstr::Move { dst, src: rhs_reg })?;
                        ctx.emit(then_block, IrInstr::Jump { target: end_block })?;

                        ctx.current_block = Some(end_block);
                        Ok(dst)
                    }
                    BinaryOperator::LogicalOr => {
                        // Short-circuit: if left is true => result true, else evaluate right
                        let then_block = ctx.new_block()?;
                        let end_block = ctx.new_block()?;
                        let dst = ctx.new_register()?;
                        let cur_block = ctx.current_block()?;

                        ctx.load_bool(cur_block, dst, true)?;
                        ctx.emit(cur_block, IrInstr::JumpIfTrue { cond: left_reg, target: end_block })?;
                        ctx.emit(cur_block, IrInstr::Jump { target: then_block })?;

                        ctx.current_block = Some(then_block);
                        let rhs_reg = right.lower(ctx)?;
                        ctx.emit(then_block, IrInstr::Move { dst, src: rhs_reg })?;
                        ctx.emit(then_block, IrInstr::Jump { target: end_block })?;

                        ctx.current_block = Some(end_block);
                        Ok(dst)
                    }
                    _ => Err(CompileError::new("E000", format!("Binary operator not implemented: {}", op))),
                }
            }
            bsharp_syntax::expressions::expression::Expression::Indexing(expression) => {
                let _target_register = (**expression).target.lower(ctx)?;
                let _index_register = (**expression).index.lower(ctx)?;
                Err(CompileError::new("E000", "Indexing expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::PostfixUnary { op, expr } => {
                let _operand_register = expr.lower(ctx)?;
                Err(CompileError::new("E000", "Postfix unary expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::This => {
                Err(CompileError::new("E000", "This expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::Base => {
                Err(CompileError::new("E000", "Base expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::Lambda(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::AnonymousMethod(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Await(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Query(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::SwitchExpression(expression) => {
                let _expression_register = (**expression).expression.lower(ctx)?;
                for arm in &(**expression).arms {
                    let _pattern_register = arm.pattern.lower(ctx).map(|_| ())?;
                    if let Some(when_clause) = &arm.when_clause {
                        let _when_register = when_clause.lower(ctx)?;
                    }
                    let _arm_expression_register = arm.expression.lower(ctx)?;
                }
                Err(CompileError::new("E000", "Switch expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::IsPattern { expression, pattern } => {
                let _expression_register = expression.lower(ctx)?;
                pattern.lower(ctx).map(|_| ())?;
                Err(CompileError::new("E000", "Is pattern expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::As { expression, target_type } => {
                let _expression_register = expression.lower(ctx)?;
                Err(CompileError::new("E000", "As expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::Cast { expression, target_type } => {
                let _expression_register = expression.lower(ctx)?;
                Err(CompileError::new("E000", "Cast expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::Throw(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Nameof(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Typeof(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Sizeof(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Default(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::StackAlloc(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Ref(expression) => {
                let _expression_register = expression.lower(ctx)?;
                Err(CompileError::new("E000", "Ref expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::Checked(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::Unchecked(expression) => expression.lower(ctx),
            bsharp_syntax::expressions::expression::Expression::With { target, initializers } => {
                let _target_register = target.lower(ctx)?;
                for initializer in initializers {
                    match initializer {
                        bsharp_syntax::expressions::expression::WithInitializerEntry::Property { name: _, value } => {
                            let _value_register = value.lower(ctx)?;
                        }
                        bsharp_syntax::expressions::expression::WithInitializerEntry::Indexer { indices, value } => {
                            for index in indices {
                                let _index_register = index.lower(ctx)?;
                            }
                            let _value_register = value.lower(ctx)?;
                        }
                    }
                }
                Err(CompileError::new("E000", "With expression lowering not implemented"))
            }
            bsharp_syntax::expressions::expression::Expression::Collection(elements) => {
                for element in elements {
                    match element {
                        bsharp_syntax::expressions::expression::CollectionElement::Expr(expr) => {
                            let _expr_register = expr.lower(ctx)?;
                        }
                        bsharp_syntax::expressions::expression::CollectionElement::Spread(expr) => {
                            let _expr_register = expr.lower(ctx)?;
                        }
                    }
                }
                Err(CompileError::new("E000", "Collection expression lowering not implemented"))
            }
        }
    }
}


