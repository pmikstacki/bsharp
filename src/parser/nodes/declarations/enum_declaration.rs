use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use super::{AttributeList, Modifier}; // Need EnumMember definition later

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnumDeclaration<'a> {
    pub attributes: Vec<AttributeList<'a>>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    // TODO: Add underlying type, members
    // pub members: Vec<EnumMember> 
}
