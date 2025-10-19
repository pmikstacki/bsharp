use crate::Identifier;
use crate::expressions::Expression;
use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub initializer: Option<Expression>,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalVariableDeclaration {
    pub is_const: bool,
    pub is_ref: bool,
    pub declaration_type: Type,
    pub declarators: Vec<VariableDeclaration>,
}
