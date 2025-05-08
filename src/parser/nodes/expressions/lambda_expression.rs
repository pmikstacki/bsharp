use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LambdaExpression<'a> {
    pub parameters: Vec<LambdaParameter<'a>>,
    pub body: LambdaBody<'a>,
    pub is_async: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LambdaParameter<'a> {
    pub name: Identifier,
    pub ty: Option<Type<'a>>,
    pub modifier: Option<LambdaParameterModifier>,
    // This marker helps Rust understand that we're intentionally using this lifetime
    #[serde(skip)]
    _phantom: PhantomData<&'a ()>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LambdaParameterModifier {
    Ref,
    Out,
    In,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LambdaBody<'a> {
    ExpressionSyntax(Expression<'a>),
    Block(Vec<Expression<'a>>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnonymousMethodExpression<'a> {
    pub parameters: Vec<LambdaParameter<'a>>,
    pub body: LambdaBody<'a>,
    pub is_async: bool,
}
