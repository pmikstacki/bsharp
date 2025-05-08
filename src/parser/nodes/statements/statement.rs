use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression; // Use absolute path
use crate::parser::nodes::declarations::LocalVariableDeclaration;
use super::{IfStatement, WhileStatement, ForStatement, BreakStatement, ContinueStatement, DoWhileStatement, SwitchStatement, ForEachStatement, TryStatement, GotoStatement, GotoCaseStatement, LabelStatement, CheckedStatement, UncheckedStatement, LockStatement, UsingStatement, YieldStatement, UnsafeStatement, FixedStatement}; // Use items from same directory's mod.rs

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Statement<'a> {
    Goto(GotoStatement),
    GotoCase(GotoCaseStatement<'a>),
    Label(LabelStatement),
    Checked(Box<CheckedStatement<'a>>),
    Unchecked(Box<UncheckedStatement<'a>>),
    Lock(Box<LockStatement<'a>>),
    Using(Box<UsingStatement<'a>>),
    Yield(YieldStatement<'a>),
    Unsafe(Box<UnsafeStatement<'a>>),
    Fixed(Box<FixedStatement<'a>>),
    Try(Box<TryStatement<'a>>),
    ForEach(Box<ForEachStatement<'a>>),
    Switch(Box<SwitchStatement<'a>>),
    DoWhile(Box<DoWhileStatement<'a>>),
    Break(BreakStatement),
    Continue(ContinueStatement),
    For(Box<ForStatement<'a>>),
    While(Box<WhileStatement<'a>>),
    If(Box<IfStatement<'a>>),
    Declaration(LocalVariableDeclaration<'a>),
    Expression(Expression<'a>),
    Return(Option<Box<Expression<'a>>>), 
    Throw(Option<Box<Expression<'a>>>), 
    Block(Vec<Statement<'a>>), // Recursive definition
}
