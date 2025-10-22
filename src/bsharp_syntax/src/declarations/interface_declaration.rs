use super::{
    InterfaceBodyDeclaration, TypeParameterConstraintClause, attribute::AttributeList,
    modifier::Modifier,
};
use crate::Identifier;
use crate::types::{Type, TypeParameter};
use serde::{Deserialize, Serialize};
// Changed Attribute to AttributeList

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>, // Added base types for interface inheritance
    pub body_declarations: Vec<InterfaceBodyDeclaration>, // Changed from ClassBodyDeclaration
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}
