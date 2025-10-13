use super::{
    BreakStatement, CheckedStatement, ContinueStatement, DoWhileStatement, FixedStatement,
    ForEachStatement, ForStatement, GotoCaseStatement, GotoStatement, IfStatement, LabelStatement,
    LocalFunctionStatement, LockStatement, SwitchStatement, TryStatement, UncheckedStatement,
    UnsafeStatement, UsingStatement, WhileStatement, YieldStatement,
};
use crate::declarations::LocalVariableDeclaration;
use crate::expressions::{DeconstructionExpression, Expression};
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
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
