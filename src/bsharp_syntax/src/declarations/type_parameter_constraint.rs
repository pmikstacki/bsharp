use crate::types::Type;
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeParameterConstraintClause {
    pub type_param: Identifier,
    pub constraints: Vec<TypeParameterConstraint>,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeParameterConstraint {
    ReferenceType,
    ValueType,
    Unmanaged,
    NotNull,
    Constructor,
    SpecificType(Type),
    SpecificParameter(Identifier),
}
