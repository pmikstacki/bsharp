use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeParameterConstraintClause {
    pub type_param: Identifier,
    pub constraints: Vec<TypeParameterConstraint>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeParameterConstraint {
    ReferenceType,
    ValueType,
    Unmanaged,
    NotNull,
    Constructor,
    SpecificType(Type),
    SpecificParameter(Identifier),
}
