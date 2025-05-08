use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::TypeParameter;
use super::{AttributeList, Modifier, ClassMember}; // Assuming InterfaceMember is similar to ClassMember initially

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterfaceDeclaration<'a> {
    pub attributes: Vec<AttributeList<'a>>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    // TODO: Add constraints, base interfaces, members
    pub members: Vec<ClassMember<'a>> // Placeholder, needs InterfaceMember later
}
