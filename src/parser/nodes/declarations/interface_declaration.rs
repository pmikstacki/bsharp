use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{TypeParameter, Type};
use super::{modifier::Modifier, attribute::AttributeList, InterfaceBodyDeclaration}; // Changed Attribute to AttributeList

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>, // Added base types for interface inheritance
    pub body_declarations: Vec<InterfaceBodyDeclaration>, // Changed from ClassBodyDeclaration
}
