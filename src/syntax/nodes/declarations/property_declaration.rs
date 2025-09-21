use crate::syntax::nodes::declarations::Modifier;
use crate::syntax::nodes::declarations::attribute::AttributeList;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::Type;
use crate::syntax::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

/// Represents a getter or setter accessors for a property
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PropertyAccessor {
    Get { modifiers: Vec<Modifier>, attributes: Vec<AttributeList>, body: Option<Statement> },
    Set { modifiers: Vec<Modifier>, attributes: Vec<AttributeList>, body: Option<Statement> },
    Init { modifiers: Vec<Modifier>, attributes: Vec<AttributeList>, body: Option<Statement> },
}

/// Represents a C# property declaration
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PropertyDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>, // Added modifiers
    pub ty: Type,
    pub name: Identifier,
    pub accessors: Vec<PropertyAccessor>,
    pub initializer: Option<Expression>, // For auto-property initializers: "public int Count { get; set; } = 0;"
}
