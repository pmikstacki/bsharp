use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::Type;
use super::{modifier::Modifier, attribute::AttributeList};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnumMember {
    pub attributes: Vec<AttributeList>,
    pub name: Identifier,
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnumDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub underlying_type: Option<Type>,
    pub enum_members: Vec<EnumMember>,
}
