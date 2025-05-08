use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::identifier::Identifier;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeParameterConstraintClause<'a> {
    pub type_param: Identifier,
    pub constraints: Vec<TypeParameterConstraint<'a>>,
    // This marker helps Rust understand that we're intentionally using this lifetime
    #[serde(skip)]
    pub _phantom: PhantomData<&'a ()>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeParameterConstraint<'a> {
    ReferenceType,
    ValueType,
    Unmanaged,
    NotNull,
    Constructor,
    SpecificType(Type<'a>),
    SpecificParameter(Identifier),
    // This variant uses the lifetime to satisfy the compiler
    #[serde(skip)]
    Phantom(PhantomData<&'a ()>),
}
