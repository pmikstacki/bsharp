use crate::declarations::LocalVariableDeclaration;
use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ForInitializer {
    Declaration(LocalVariableDeclaration),
    Expressions(Vec<Expression>), // Represents comma-separated expression statements
}
