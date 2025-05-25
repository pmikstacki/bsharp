use crate::parser::nodes::expressions::indexing_expression::IndexingExpression;
use crate::parser::nodes::expressions::literal::Literal;
use crate::parser::nodes::expressions::BinaryOperator;
use crate::parser::nodes::expressions::UnaryOperator;
use crate::parser::nodes::expressions::{
    AnonymousMethodExpression, AnonymousObjectCreationExpression, AssignmentExpression, AwaitExpression,
    ConditionalExpression, DeconstructionExpression, InvocationExpression, LambdaExpression, MemberAccessExpression,
    NewExpression, Pattern, TupleExpression
};
use crate::parser::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expression {
    AnonymousObject(AnonymousObjectCreationExpression),
    Tuple(TupleExpression),
    Pattern(Box<Pattern>),
    Deconstruction(DeconstructionExpression),
    Conditional(Box<ConditionalExpression>),
    New(Box<NewExpression>),
    MemberAccess(Box<MemberAccessExpression>),
    Invocation(Box<InvocationExpression>),
    Assignment(Box<AssignmentExpression>),
    Literal(Literal),
    Variable(Identifier),
    Unary { op: UnaryOperator, expr: Box<Expression> },
    Binary { left: Box<Expression>, op: BinaryOperator, right: Box<Expression> },
    Indexing(Box<IndexingExpression>),
    PostfixUnary {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    This, // Added 'this' keyword expression
    Base, // Added 'base' keyword expression
    Lambda(Box<LambdaExpression>), // Lambda expressions: x => x * 2
    AnonymousMethod(Box<AnonymousMethodExpression>), // Anonymous methods: delegate(int x) { return x * 2; }
    Await(Box<AwaitExpression>), // Await expressions: await SomeMethodAsync()
    // TODO: Add variants for other C# expressions as needed:
    // e.g., Cast, TypeOf, Default, Query, etc.
}
