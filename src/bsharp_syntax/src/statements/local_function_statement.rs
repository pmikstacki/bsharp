use crate::declarations::{Modifier, TypeParameterConstraintClause};
use crate::statements::statement::Statement;
use crate::types::{Parameter, Type, TypeParameter};
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalFunctionStatement {
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub parameters: Vec<Parameter>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
    pub body: Box<Statement>,
}
