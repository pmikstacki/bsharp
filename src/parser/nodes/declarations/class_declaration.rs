use serde::{Serialize, Deserialize};
use crate::parser::nodes::Identifier; // Updated path
use crate::parser::nodes::XmlDocumentationComment; // Added for documentation
use crate::parser::nodes::declarations::{ClassBodyDeclaration, Modifier, attribute::AttributeList};
use crate::parser::nodes::types::{TypeParameter, Type};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>,
    pub body_declarations: Vec<ClassBodyDeclaration>,
    pub documentation: Option<XmlDocumentationComment>,
}
