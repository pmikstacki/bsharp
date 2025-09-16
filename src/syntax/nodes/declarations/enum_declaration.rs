use super::{attribute::AttributeList, modifier::Modifier};
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

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
