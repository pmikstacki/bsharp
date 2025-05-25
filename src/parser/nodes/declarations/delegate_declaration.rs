use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{TypeParameter, Type, Parameter}; // Need Parameter too
use super::{AttributeList, Modifier};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DelegateDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    // TODO: Add constraints
}
