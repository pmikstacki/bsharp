use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::Type;
use crate::syntax::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LambdaExpression {
    pub parameters: Vec<LambdaParameter>,
    pub body: LambdaBody,
    pub is_async: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LambdaParameter {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub modifier: Option<LambdaParameterModifier>,
    // This marker helps Rust understand that we're intentionally using this lifetime

}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LambdaParameterModifier {
    Ref,
    Out,
    In,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LambdaBody {
    ExpressionSyntax(Expression),
    Block(Vec<Statement>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnonymousMethodExpression {
    pub parameters: Vec<LambdaParameter>,
    pub body: LambdaBody,
    pub is_async: bool,
}
