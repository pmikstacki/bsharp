use crate::syntax::nodes::declarations::{Attribute, Modifier};
use crate::syntax::nodes::types::Parameter;
use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

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
