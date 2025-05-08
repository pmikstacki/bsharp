use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{TypeParameter, Type, Parameter}; // Need Parameter too
use super::{AttributeList, Modifier};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DelegateDeclaration<'a> {
    pub attributes: Vec<AttributeList<'a>>,
    pub modifiers: Vec<Modifier>,
    pub return_type: Type<'a>,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter<'a>>,
    // TODO: Add constraints
}
