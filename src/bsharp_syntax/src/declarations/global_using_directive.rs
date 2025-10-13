use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GlobalUsingDirective {
    pub using_directive: super::UsingDirective, // Reuse the existing UsingDirective enum/struct
}
