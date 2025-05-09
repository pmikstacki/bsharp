use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{TypeParameter, Type};
use super::attribute::AttributeList;
use super::modifier::Modifier;
use super::field_declaration::FieldDeclaration;
use super::method_declaration::MethodDeclaration;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum StructMember<'a> {
    Field(FieldDeclaration<'a>),
    Method(MethodDeclaration<'a>),
    // TODO: Add other struct members like constructors, properties, nested types, etc.
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructDeclaration<'a> {
    pub attributes: Vec<AttributeList<'a>>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type<'a>>,
    pub members: Vec<StructMember<'a>>,
}
