use serde::{Serialize, Deserialize};
use crate::parser::nodes::declarations::LocalVariableDeclaration;
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ForInitializer {
    Declaration(LocalVariableDeclaration),
    Expressions(Vec<Expression>), // Represents comma-separated expression statements
}
