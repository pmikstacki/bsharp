use serde::{Serialize, Deserialize};
use crate::parser::nodes::declarations::Attribute;
use crate::parser::nodes::types::{Type, Parameter};
use crate::parser::nodes::identifier::Identifier;
use super::Modifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordDeclaration<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub is_struct: bool, // Added: true for 'record struct', false for 'record class'
    pub parameters: Vec<Parameter<'a>>, // primary constructor
    pub base_types: Vec<Type<'a>>,
    pub members: Vec<super::ClassMember<'a>>, // member declarations (fields, properties, etc.)
}
