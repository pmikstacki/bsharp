use crate::syntax::nodes::declarations::modifier::Modifier;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::types::Parameter;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConstructorDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: Identifier, // Constructor name is same as class name
    pub parameters: Vec<Parameter>,
    pub body: Option<Statement>,
    // Add attributes, etc. later if needed
}
