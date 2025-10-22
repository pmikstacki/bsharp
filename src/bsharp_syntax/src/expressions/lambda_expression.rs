use crate::Identifier;
use crate::expressions::Expression;
use crate::statements::statement::Statement;
use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LambdaExpression {
    pub parameters: Vec<LambdaParameter>,
    pub body: LambdaBody,
    pub is_async: bool,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LambdaParameter {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub modifier: Option<LambdaParameterModifier>,
    // This marker helps Rust understand that we're intentionally using this lifetime
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LambdaParameterModifier {
    Ref,
    Out,
    In,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LambdaBody {
    ExpressionSyntax(Expression),
    Block(Vec<Statement>),
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnonymousMethodExpression {
    pub parameters: Vec<LambdaParameter>,
    pub body: LambdaBody,
    pub is_async: bool,
}
