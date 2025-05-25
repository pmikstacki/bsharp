use super::attribute::AttributeList;
use super::constructor_declaration::ConstructorDeclaration;
use super::field_declaration::FieldDeclaration;
use super::method_declaration::MethodDeclaration;
use super::modifier::Modifier;
use super::property_declaration::PropertyDeclaration;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{Type, TypeParameter};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum StructBodyDeclaration {
    Field(FieldDeclaration),
    Method(MethodDeclaration),
    Property(PropertyDeclaration),
    Constructor(ConstructorDeclaration),
    // TODO: Add other struct members like nested types, etc.
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>,
    pub body_declarations: Vec<StructBodyDeclaration>,
}
