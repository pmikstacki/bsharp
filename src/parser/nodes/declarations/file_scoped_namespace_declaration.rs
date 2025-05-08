use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::declarations::namespace_declaration::NamespaceMember;
use super::UsingDirective;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileScopedNamespaceDeclaration<'a> {
    pub name: Identifier, // The namespace name (e.g., MyOrg.MyApp)
    pub usings: Vec<UsingDirective>,
    pub members: Vec<NamespaceMember<'a>>, // Declarations within this file-scoped namespace
}
