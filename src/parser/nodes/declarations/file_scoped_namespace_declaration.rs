use crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration;
use crate::parser::nodes::declarations::UsingDirective;
use crate::parser::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileScopedNamespaceDeclaration {
    pub name: Identifier, // The namespace name (e.g., MyOrg.MyApp)
    pub declarations: Vec<NamespaceBodyDeclaration>, // Declarations within this file-scoped namespace
    pub using_directives: Vec<UsingDirective>,
}
