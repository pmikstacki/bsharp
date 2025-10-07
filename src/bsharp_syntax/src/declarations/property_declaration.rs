use crate::declarations::{AttributeList, Modifier};
use crate::expressions::Expression;
use crate::statements::statement::Statement;
use crate::types::Type;
use crate::Identifier;
use serde::{Deserialize, Serialize};

/// Represents a getter or setter accessors for a property
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PropertyAccessor {
    Get {
        modifiers: Vec<Modifier>,
        attributes: Vec<AttributeList>,
        body: Option<Statement>,
    },
    Set {
        modifiers: Vec<Modifier>,
        attributes: Vec<AttributeList>,
        body: Option<Statement>,
    },
    Init {
        modifiers: Vec<Modifier>,
        attributes: Vec<AttributeList>,
        body: Option<Statement>,
    },
}

/// Represents a C# property declaration
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PropertyDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>, // Added modifiers
    pub property_type: Type,
    pub name: Identifier,
    pub accessors: Vec<PropertyAccessor>,
    pub initializer: Option<Expression>, // For auto-property initializers: "public int Count { get; set; } = 0;"
}
