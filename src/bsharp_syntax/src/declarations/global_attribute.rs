use crate::declarations::Attribute;
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GlobalAttribute {
    pub target: Identifier, // e.g. "assembly", "module"
    pub attribute: Attribute,
}
