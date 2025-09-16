use crate::syntax::nodes::declarations::Attribute;
use crate::syntax::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GlobalAttribute {
    pub target: Identifier, // e.g. "assembly", "module"
    pub attribute: Attribute,
}
