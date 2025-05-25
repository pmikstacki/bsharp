use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression; // Use absolute path
use crate::parser::nodes::declarations::LocalVariableDeclaration;
use super::{IfStatement, WhileStatement, ForStatement, BreakStatement, ContinueStatement, DoWhileStatement, SwitchStatement, ForEachStatement, TryStatement, GotoStatement, GotoCaseStatement, LabelStatement, CheckedStatement, UncheckedStatement, LockStatement, UsingStatement, YieldStatement, UnsafeStatement, FixedStatement}; // Use items from same directory's mod.rs

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
    Expression(Expression),
    Return(Option<Box<Expression>>), 
    Throw(Option<Box<Expression>>), 
    Block(Vec<Statement>), // Recursive definition
    Empty, // Added Empty variant for empty statements
}
