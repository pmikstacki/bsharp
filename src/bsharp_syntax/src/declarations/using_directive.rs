use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum UsingDirective {
    Namespace {
        namespace: Identifier,
    },
    Alias {
        alias: Identifier,
        namespace_or_type: Identifier,
    },
    Static {
        type_name: Identifier,
    },
}
