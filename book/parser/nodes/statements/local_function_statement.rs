use crate::parser::nodes::declarations::{Modifier, TypeParameterConstraintClause};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::types::{Parameter, Type, TypeParameter};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalFunctionStatement {
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub parameters: Vec<Parameter>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
    pub body: Box<Statement>,
} 