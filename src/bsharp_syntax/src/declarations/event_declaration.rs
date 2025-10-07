use crate::declarations::{AttributeList, Modifier};
use crate::statements::statement::Statement;
use crate::types::Type;
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub event_type: Type,
    pub name: Identifier,
    pub accessor_list: Option<EventAccessorList>, // for event { add; remove; }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventAccessorList {
    pub add_accessor: Option<EventAccessor>,
    pub remove_accessor: Option<EventAccessor>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventAccessor {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub body: Option<Statement>, // None for interface events or abstract events
}
