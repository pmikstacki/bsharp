use crate::parser::nodes::declarations::attribute::AttributeList;
use crate::parser::nodes::declarations::modifier::Modifier;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{Parameter, Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub is_struct: bool, // true for "record struct", false for "record class"
    pub parameters: Option<Vec<Parameter>>, // For positional records
    pub base_types: Vec<Type>,
    pub body_declarations: Vec<super::ClassBodyDeclaration>, // member declarations (fields, properties, etc.)
}
