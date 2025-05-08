use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VariableDeclarator<'a> {
    pub name: Identifier,
    pub initializer: Option<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalVariableDeclaration<'a> {
    pub is_const: bool,
    pub ty: Type<'a>,
    pub declarators: Vec<VariableDeclarator<'a>>,
}
