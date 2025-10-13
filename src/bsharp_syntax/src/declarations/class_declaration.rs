use crate::declarations::{
    AttributeList, ClassBodyDeclaration, Modifier, TypeParameterConstraintClause,
};
use crate::trivia::xml_documentation::XmlDocumentationComment;
use crate::types::{Parameter, Type, TypeParameter};
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    /// C# 12 primary constructor parameters: class Name(int X, int Y)
    pub primary_constructor_parameters: Option<Vec<Parameter>>,
    pub base_types: Vec<Type>,
    pub body_declarations: Vec<ClassBodyDeclaration>,
    pub documentation: Option<XmlDocumentationComment>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}
