use serde::{Serialize, Deserialize};
use crate::parser::nodes::declarations::Attribute;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GlobalAttribute<'a> {
    pub target: Identifier, // e.g. "assembly", "module"
    pub attribute: Attribute<'a>,
}
