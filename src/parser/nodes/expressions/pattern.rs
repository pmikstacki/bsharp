use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern<'a> {
    Declaration { target_type: Type<'a>, name: Identifier },
    Constant(Expression<'a>),
    Var(Identifier),
    Recursive { target_type: Option<Type<'a>>, subpatterns: Vec<Pattern<'a>> }, // for property/positional
    Property { name: Identifier, pattern: Box<Pattern<'a>> },
    Positional(Vec<Pattern<'a>>),
    Relational { op: Identifier, value: Expression<'a> },
    LogicalAnd(Box<Pattern<'a>>, Box<Pattern<'a>>),
    LogicalOr(Box<Pattern<'a>>, Box<Pattern<'a>>),
    Not(Box<Pattern<'a>>),
    Parenthesized(Box<Pattern<'a>>),
    Discard, // _
    // This variant uses the lifetime to satisfy the compiler
    #[serde(skip)]
    Phantom(PhantomData<&'a ()>),
}

use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PatternCase<'a> {
    pub pattern: Pattern<'a>,
    pub when_clause: Option<Expression<'a>>,
    pub body: Vec<Expression<'a>>,
}
