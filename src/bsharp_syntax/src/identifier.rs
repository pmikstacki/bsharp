use crate::expressions::{BinaryOperator, UnaryOperator};
use bsharp_syntax_derive::AstNode;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(AstNode, Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Identifier {
    Simple(String),
    QualifiedIdentifier(Vec<String>),
    OperatorOverrideIdentifier(OverrideOperatorType),
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identifier::Simple(name) => write!(f, "{}", name),
            Identifier::QualifiedIdentifier(segments) => write!(f, "{}", segments.join(".")),
            Identifier::OperatorOverrideIdentifier(operator_type) => match operator_type {
                OverrideOperatorType::Unary(operator) => write!(f, "{}", operator),
                OverrideOperatorType::Binary(operator) => write!(f, "{}", operator),
                OverrideOperatorType::Boolean(value) => write!(f, "bool ({})", value),
            },
        }
    }
}

#[derive(AstNode, Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum OverrideOperatorType {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
    Boolean(bool),
}

impl Identifier {
    pub fn new(name: impl Into<String>) -> Self {
        let s: String = name.into();
        if s.contains('.') {
            let segs: Vec<String> = s.split('.').map(|p| p.to_string()).collect();
            Identifier::QualifiedIdentifier(segs)
        } else {
            Identifier::Simple(s)
        }
    }
}
