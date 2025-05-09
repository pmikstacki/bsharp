use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{Type, TypeParameter, Parameter};
use crate::parser::nodes::declarations::{Modifier, TypeParameterConstraintClause};
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MethodDeclaration<'a> {
    pub modifiers: Vec<Modifier>,
    pub return_type: Type<'a>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub parameters: Vec<Parameter<'a>>,
    pub body: Option<String>,
    pub constraints: Vec<TypeParameterConstraintClause<'a>>,
    // This marker helps Rust understand that we're intentionally using this lifetime
    #[serde(skip)]
    pub _phantom: PhantomData<&'a ()>,
}

// Note: The following structs and enum were added in the provided code edit, 
// but they were not present in the original code document. 
// They are included here for completeness, but they may not be relevant to the original code.

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeParameterConstraint<'a> {
    pub type_parameter: Identifier,
    pub constraints: Vec<ConstraintType<'a>>,
    // This marker helps Rust understand that we're intentionally using this lifetime
    #[serde(skip)]
    _phantom: PhantomData<&'a ()>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConstraintType<'a> {
    Class,
    Struct,
    Unmanaged,
    // We need a variant that uses the lifetime to satisfy the compiler
    #[serde(skip)]
    Phantom(PhantomData<&'a ()>),
    New,
    Type(Type<'a>),
}
