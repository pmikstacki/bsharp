use crate::syntax::nodes::Identifier;
use serde::{Deserialize, Serialize};
// Updated path
use crate::syntax::nodes::XmlDocumentationComment;
// Added for documentation
use crate::syntax::nodes::declarations::{attribute::AttributeList, ClassBodyDeclaration, Modifier};
use crate::syntax::nodes::types::{Type, TypeParameter};

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
