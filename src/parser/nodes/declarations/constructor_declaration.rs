use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Parameter;
use crate::parser::nodes::declarations::Attribute;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConstructorDeclaration<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub modifiers: Vec<String>,
    pub name: Identifier,
    pub parameters: Vec<Parameter<'a>>,
    pub initializer: Option<ConstructorInitializer>,
    pub body: String, // body or signature
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConstructorInitializer {
    Base(Vec<String>), // arguments
    This(Vec<String>),
}
