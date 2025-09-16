use crate::syntax::nodes::declarations::LocalVariableDeclaration;
use crate::syntax::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ForInitializer {
    Declaration(LocalVariableDeclaration),
    Expressions(Vec<Expression>), // Represents comma-separated expression statements
}
