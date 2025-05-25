use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::types::Parameter;
use crate::parser::nodes::declarations::{Attribute, Modifier};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexerDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub indexer_type: Type,
    pub parameters: Vec<Parameter>,
    pub accessor_list: IndexerAccessorList,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexerAccessorList {
    pub get_accessor: Option<String>, // body or signature
    pub set_accessor: Option<String>, // body or signature
}
