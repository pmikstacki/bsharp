use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Declaration { target_type: Type, name: Identifier },
    Constant(Expression),
    Var(Identifier),
    Recursive { target_type: Option<Type>, subpatterns: Vec<Pattern> }, // for property/positional
    Property { name: Identifier, pattern: Box<Pattern> },
    Positional(Vec<Pattern>),
    Relational { op: Identifier, value: Expression },
    LogicalAnd(Box<Pattern>, Box<Pattern>),
    LogicalOr(Box<Pattern>, Box<Pattern>),
    Not(Box<Pattern>),
    Parenthesized(Box<Pattern>),
    Discard, // _
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PatternCase {
    pub pattern: Pattern,
    pub when_clause: Option<Expression>,
    pub body: Vec<Expression>,
}
