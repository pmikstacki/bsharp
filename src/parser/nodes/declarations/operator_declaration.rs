use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::types::Parameter;
use crate::parser::nodes::declarations::Attribute;
use crate::parser::nodes::identifier::Identifier;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OperatorDeclaration<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub modifiers: Vec<String>,
    pub return_type: Type<'a>,
    pub operator: OperatorKind<'a>,
    pub parameters: Vec<Parameter<'a>>,
    pub body: String, // body or signature
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum OperatorKind<'a> {
    Unary(Identifier), // op symbol
    Binary(Identifier),
    Conversion { kind: ConversionKind, target_type: Type<'a> },
    // This variant uses the lifetime to satisfy the compiler
    #[serde(skip)]
    Phantom(PhantomData<&'a ()>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConversionKind {
    Implicit,
    Explicit,
}
