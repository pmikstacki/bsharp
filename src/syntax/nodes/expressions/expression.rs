use crate::syntax::nodes::expressions::indexing_expression::IndexingExpression;
use crate::syntax::nodes::expressions::literal::Literal;
use crate::syntax::nodes::expressions::range_expression::{IndexExpression, RangeExpression};
use crate::syntax::nodes::expressions::BinaryOperator;
use crate::syntax::nodes::expressions::UnaryOperator;
use crate::syntax::nodes::expressions::{
    AnonymousMethodExpression, AnonymousObjectCreationExpression, AssignmentExpression,
    AwaitExpression, ConditionalExpression, DeconstructionExpression, DefaultExpression,
    InvocationExpression, LambdaExpression, MemberAccessExpression, NameofExpression,
    NewExpression, NullConditionalExpression, Pattern, QueryExpression, SizeofExpression,
    StackAllocExpression, ThrowExpression, TupleExpression, TypeofExpression,
};
use crate::syntax::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expression {
    AnonymousObject(AnonymousObjectCreationExpression),
    Tuple(TupleExpression),
    Range(Box<RangeExpression>), // Range expressions: start..end, ..end, start.., ..
    Index(Box<IndexExpression>), // Index from end expressions: ^expression
    Pattern(Box<Pattern>),
    Deconstruction(DeconstructionExpression),
    Conditional(Box<ConditionalExpression>),
    New(Box<NewExpression>),
    MemberAccess(Box<MemberAccessExpression>),
    NullConditional(Box<NullConditionalExpression>),
    Invocation(Box<InvocationExpression>),
    Assignment(Box<AssignmentExpression>),
    Literal(Literal),
    Variable(Identifier),
    Unary {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    Indexing(Box<IndexingExpression>),
    PostfixUnary {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    This,                                            // Added 'this' keyword expression
    Base,                                            // Added 'base' keyword expression
    Lambda(Box<LambdaExpression>),                   // Lambda expressions: x => x * 2
    AnonymousMethod(Box<AnonymousMethodExpression>), // Anonymous methods: delegate(int x) { return x * 2; }
    Await(Box<AwaitExpression>),                     // Await expressions: await SomeMethodAsync()
    Query(Box<QueryExpression>), // LINQ query expressions: from x in collection select x
    SwitchExpression(Box<SwitchExpression>), // Switch expressions: x switch { 1 => "one", _ => "other" }
    IsPattern {
        expression: Box<Expression>,
        pattern: Box<Pattern>,
    }, // Pattern matching: x is int y
    Cast {
        expression: Box<Expression>,
        target_type: crate::syntax::nodes::types::Type,
    }, // Type casting: (int)x
    Throw(Box<ThrowExpression>),             // Throw expressions: throw new Exception()
    Nameof(Box<NameofExpression>),           // Nameof expressions: nameof(variable)
    Typeof(Box<TypeofExpression>),           // Typeof expressions: typeof(int)
    Sizeof(Box<SizeofExpression>),           // Sizeof expressions: sizeof(int)
    Default(Box<DefaultExpression>),         // Default expressions: default(int) or default
    StackAlloc(Box<StackAllocExpression>),   // Stackalloc expressions: stackalloc int[10]
    Ref(Box<Expression>),                    // Ref expressions: ref field, ref array[index]
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchExpression {
    pub expression: Expression,
    pub arms: Vec<SwitchExpressionArm>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchExpressionArm {
    pub pattern: Pattern,
    pub when_clause: Option<Expression>,
    pub expression: Expression,
}
