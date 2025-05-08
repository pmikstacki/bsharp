use serde::{Serialize, Deserialize};
// Use the newly structured node locations
use super::nodes::declarations::{UsingDirective, NamespaceDeclaration, ClassDeclaration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile<'a> {
    pub usings: Vec<UsingDirective>,
    pub members: Vec<TopLevelMember<'a>>, // Can be Namespace or Class/Struct/etc.
}

// Define what can appear at the top level of a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopLevelMember<'a> {
    Namespace(NamespaceDeclaration<'a>),
    Class(ClassDeclaration<'a>),
    // TODO: Add other top-level members like Struct, Enum, Interface later
}
