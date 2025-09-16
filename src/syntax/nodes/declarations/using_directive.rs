use crate::syntax::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum UsingDirective {
    Namespace { namespace: Identifier }, 
    Alias { alias: Identifier, namespace_or_type: Identifier }, 
    Static { type_name: Identifier }, 
}
