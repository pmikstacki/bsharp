use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::declarations::{ClassMember, Modifier};
use crate::parser::nodes::types::TypeParameter;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassDeclaration<'a> {
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub members: Vec<ClassMember<'a>>,
}
