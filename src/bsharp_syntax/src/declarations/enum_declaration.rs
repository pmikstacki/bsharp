use super::{attribute::AttributeList, modifier::Modifier};
use crate::Identifier;
use crate::expressions::Expression;
use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnumMember {
    pub attributes: Vec<AttributeList>,
    pub name: Identifier,
    pub value: Option<Expression>,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnumDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub underlying_type: Option<Type>,
    pub enum_members: Vec<EnumMember>,
}
