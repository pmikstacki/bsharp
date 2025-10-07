use crate::declarations::{Attribute, Modifier};
use crate::types::{Parameter, Type};
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OperatorDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub operator: OperatorKind,
    pub parameters: Vec<Parameter>,
    pub body: String, // body or signature
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum OperatorKind {
    Unary(Identifier), // op symbol
    Binary(Identifier),
    Conversion {
        kind: ConversionKind,
        target_type: Type,
    },
    // This variant uses the lifetime to satisfy the compiler
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConversionKind {
    Implicit,
    Explicit,
}
