use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{Parameter, Type, TypeParameter};
use serde::{Deserialize, Serialize};
// Need Parameter too
use super::{AttributeList, Modifier, TypeParameterConstraintClause};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DelegateDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}
