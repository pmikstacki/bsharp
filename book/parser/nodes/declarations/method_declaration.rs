use crate::parser::nodes::declarations::{Modifier, TypeParameterConstraintClause};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::types::{Parameter, Type, TypeParameter};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MethodDeclaration {
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub parameters: Vec<Parameter>,
    pub body: Option<Statement>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeParameterConstraint {
    pub parameter_name: Identifier,
    pub constraint_type: ConstraintType,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    Class,
    Struct,
    Unmanaged,
    // We need a variant that uses the lifetime to satisfy the compiler
    // (Removed Phantom variant; no longer needed after lifetime removal)
    New,
    Type(Type),
}
