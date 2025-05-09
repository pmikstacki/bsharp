use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::Type;
use super::{Attribute, Modifier};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnumMember<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub name: Identifier,
    pub value: Option<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnumDeclaration<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub underlying_type: Option<Type<'a>>,
    pub members: Vec<EnumMember<'a>>,
}
