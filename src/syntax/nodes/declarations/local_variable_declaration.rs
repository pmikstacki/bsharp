use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub name: Identifier,
    pub initializer: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalVariableDeclaration {
    pub is_const: bool,
    pub is_ref: bool,
    pub declaration_type: Type,
    pub declarators: Vec<VariableDeclarator>,
}
