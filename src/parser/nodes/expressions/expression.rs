use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::expressions::BinaryOperator;
use crate::parser::nodes::expressions::literal::Literal;
use crate::parser::nodes::expressions::{
    AssignmentExpression, ConditionalExpression, InvocationExpression, MemberAccessExpression,
    NewExpression, AnonymousObjectCreationExpression, TupleExpression, Pattern, DeconstructionExpression
};
use crate::parser::nodes::expressions::UnaryOperator;
use crate::parser::nodes::expressions::indexing_expression::IndexingExpression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expression<'a> {
    AnonymousObject(AnonymousObjectCreationExpression<'a>),
    Tuple(TupleExpression<'a>),
    Pattern(Box<Pattern<'a>>),
    Deconstruction(DeconstructionExpression<'a>),
    Conditional(Box<ConditionalExpression<'a>>),
    New(Box<NewExpression<'a>>),
    MemberAccess(Box<MemberAccessExpression<'a>>),
    Invocation(Box<InvocationExpression<'a>>),
    Assignment(Box<AssignmentExpression<'a>>),
    Literal(Literal),
    Variable(Identifier),
    Unary { op: UnaryOperator, expr: Box<Expression<'a>> },
    Binary { left: Box<Expression<'a>>, op: BinaryOperator, right: Box<Expression<'a>> },
    Indexing(Box<IndexingExpression<'a>>),
    PostfixUnary { op: UnaryOperator, expr: Box<Expression<'a>> },
}
