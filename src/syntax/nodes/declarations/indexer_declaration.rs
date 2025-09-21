use super::property_declaration::PropertyDeclaration;
use crate::syntax::nodes::statements::statement::Statement;
use super::{Attribute, Modifier};
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
pub struct IndexerAccessor {
    pub modifiers: Vec<Modifier>,
    pub attributes: Vec<Attribute>,
    pub body: Option<Statement>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexerAccessorList {
    pub get_accessor: Option<IndexerAccessor>,
    pub set_accessor: Option<IndexerAccessor>,
}
