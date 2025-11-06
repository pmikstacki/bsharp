use crate::declarations::ClassBodyDeclaration;
use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExtensionDeclaration {
    pub receiver: Type,
    pub members: Vec<ClassBodyDeclaration>,
}
