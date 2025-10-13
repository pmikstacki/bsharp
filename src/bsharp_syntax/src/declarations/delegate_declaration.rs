// Need Parameter too
use super::{AttributeList, Modifier, TypeParameterConstraintClause};
use crate::types::{Parameter, Type, TypeParameter};
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DelegateDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}
