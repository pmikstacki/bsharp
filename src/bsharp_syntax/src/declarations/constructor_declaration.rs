use crate::Identifier;
use crate::declarations::{ConstructorInitializer, Modifier};
use crate::statements::statement::Statement;
use crate::types::Parameter;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConstructorDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: Identifier, // Constructor name is same as class name
    pub parameters: Vec<Parameter>,
    pub body: Option<Statement>,
    pub initializer: Option<ConstructorInitializer>,
    // Add attributes, etc. later if needed
}
