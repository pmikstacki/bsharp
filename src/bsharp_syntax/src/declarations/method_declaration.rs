use crate::Identifier;
use crate::declarations::{Modifier, TypeParameterConstraintClause};
use crate::statements::statement::Statement;
use crate::types::{Parameter, Type, TypeParameter};
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MethodDeclaration {
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub parameters: Vec<Parameter>,
    pub body: Option<Statement>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeParameterConstraint {
    pub parameter_name: Identifier,
    pub constraint_type: ConstraintType,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    Class,
    Struct,
    Unmanaged,
    // We need a variant that uses the lifetime to satisfy the compiler
    // (Removed Phantom variant; no longer needed after lifetime removal)
    New,
    Type(Type),
}
