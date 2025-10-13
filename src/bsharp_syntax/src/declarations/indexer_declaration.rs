use super::{Attribute, Modifier};
use crate::statements::statement::Statement;
use crate::types::{Parameter, Type};
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexerDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub indexer_type: Type,
    pub parameters: Vec<Parameter>,
    pub accessor_list: IndexerAccessorList,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexerAccessor {
    pub modifiers: Vec<Modifier>,
    pub attributes: Vec<Attribute>,
    pub body: Option<Statement>,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexerAccessorList {
    pub get_accessor: Option<IndexerAccessor>,
    pub set_accessor: Option<IndexerAccessor>,
}
