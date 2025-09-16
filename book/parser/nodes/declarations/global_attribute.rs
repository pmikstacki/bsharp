use crate::parser::nodes::declarations::Attribute;
use crate::parser::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GlobalAttribute {
    pub target: Identifier, // e.g. "assembly", "module"
    pub attribute: Attribute,
}
