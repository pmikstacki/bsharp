use super::{
    BreakStatement, CheckedStatement, ContinueStatement, DoWhileStatement, FixedStatement,
    ForEachStatement, ForStatement, GotoCaseStatement, GotoStatement, IfStatement, LabelStatement,
    LockStatement, SwitchStatement, TryStatement, UncheckedStatement, UnsafeStatement,
    UsingStatement, WhileStatement, YieldStatement,
};
// Use absolute path
use crate::syntax::nodes::declarations::LocalVariableDeclaration;
use crate::syntax::nodes::expressions::expression::Expression;
// Use items from same directory's mod.rs
use crate::syntax::nodes::expressions::DeconstructionExpression;
use crate::syntax::nodes::statements::local_function_statement::LocalFunctionStatement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Statement {
    Goto(GotoStatement),
    GotoCase(GotoCaseStatement),
    Label(LabelStatement),
    Checked(Box<CheckedStatement>),
    Unchecked(Box<UncheckedStatement>),
    Lock(Box<LockStatement>),
    Using(Box<UsingStatement>),
    Yield(YieldStatement),
    Unsafe(Box<UnsafeStatement>),
    Fixed(Box<FixedStatement>),
    Try(Box<TryStatement>),
    ForEach(Box<ForEachStatement>),
    Switch(Box<SwitchStatement>),
    DoWhile(Box<DoWhileStatement>),
    Break(BreakStatement),
    Continue(ContinueStatement),
    For(Box<ForStatement>),
    While(Box<WhileStatement>),
    If(Box<IfStatement>),
    Declaration(LocalVariableDeclaration),
    LocalFunction(Box<LocalFunctionStatement>),
    Expression(Expression),
    Return(Option<Box<Expression>>),
    Throw(Option<Box<Expression>>),
    Block(Vec<Statement>),                         // Recursive definition
    Empty,                                         // Added Empty variant for empty statements
    Deconstruction(Box<DeconstructionExpression>), // Added for tuple deconstruction
}
