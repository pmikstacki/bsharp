use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{TypeParameter, Type};
use super::{Attribute, Modifier, ClassMember}; // Assuming InterfaceMember is similar to ClassMember initially

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterfaceDeclaration<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub base_types: Vec<Type<'a>>, // Added base types for interface inheritance
    pub members: Vec<ClassMember<'a>> // Placeholder, needs InterfaceMember later
}
