use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::Parameter;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::declarations::modifier::Modifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConstructorDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: Identifier, // Constructor name is same as class name
    pub parameters: Vec<Parameter>,
    pub body: Option<Statement>,
    // Add attributes, etc. later if needed
}
