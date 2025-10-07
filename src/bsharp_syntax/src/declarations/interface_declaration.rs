use super::{attribute::AttributeList, modifier::Modifier, InterfaceBodyDeclaration, TypeParameterConstraintClause};
use crate::types::{Type, TypeParameter};
use crate::Identifier;
use serde::{Deserialize, Serialize};
// Changed Attribute to AttributeList

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>, // Added base types for interface inheritance
    pub body_declarations: Vec<InterfaceBodyDeclaration>, // Changed from ClassBodyDeclaration
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}
