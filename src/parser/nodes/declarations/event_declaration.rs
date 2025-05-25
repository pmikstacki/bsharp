use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::declarations::Attribute;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<String>,
    pub ty: Type,
    pub name: Identifier,
    pub accessor_list: Option<EventAccessorList>, // for event { add; remove; }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventAccessorList {
    pub add_accessor: Option<String>, // body or signature
    pub remove_accessor: Option<String>, // body or signature
}
