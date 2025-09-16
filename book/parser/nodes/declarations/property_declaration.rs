use crate::parser::nodes::declarations::Modifier;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::Type;
use serde::{Deserialize, Serialize};

/// Represents a getter or setter accessors for a property
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PropertyAccessor {
    Get(Option<String>),  // String is the body content, None if it's an auto-property
    Set(Option<String>),  // String is the body content, None if it's an auto-property
    Init(Option<String>), // For init-only properties (C# 9+)
}

/// Represents a C# property declaration
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PropertyDeclaration {
    pub modifiers: Vec<Modifier>,  // Added modifiers
    pub ty: Type,
    pub name: Identifier,
    pub accessors: Vec<PropertyAccessor>,
    pub initializer: Option<Expression>, // For auto-property initializers: "public int Count { get; set; } = 0;"
}
