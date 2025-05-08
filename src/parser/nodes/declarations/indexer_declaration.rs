use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::types::Parameter;
use crate::parser::nodes::declarations::Attribute;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexerDeclaration<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub modifiers: Vec<String>,
    pub ty: Type<'a>,
    pub parameters: Vec<Parameter<'a>>,
    pub accessor_list: IndexerAccessorList,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexerAccessorList {
    pub get_accessor: Option<String>, // body or signature
    pub set_accessor: Option<String>, // body or signature
}
