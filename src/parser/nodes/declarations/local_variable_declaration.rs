use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub name: Identifier,
    pub initializer: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalVariableDeclaration {
    pub is_const: bool,
    pub declaration_type: Type,
    pub declarators: Vec<VariableDeclarator>,
}
