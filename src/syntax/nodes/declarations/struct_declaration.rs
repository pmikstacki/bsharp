use super::attribute::AttributeList;
use super::constructor_declaration::ConstructorDeclaration;
use super::field_declaration::FieldDeclaration;
use super::method_declaration::MethodDeclaration;
use super::event_declaration::EventDeclaration;
use super::indexer_declaration::IndexerDeclaration;
use super::operator_declaration::OperatorDeclaration;
use super::{ClassDeclaration, EnumDeclaration, InterfaceDeclaration, RecordDeclaration};
use super::modifier::Modifier;
use super::property_declaration::PropertyDeclaration;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::{Type, TypeParameter};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum StructBodyDeclaration {
    Field(FieldDeclaration),
    Method(MethodDeclaration),
    Property(PropertyDeclaration),
    Constructor(ConstructorDeclaration),
    Event(EventDeclaration),
    Indexer(IndexerDeclaration),
    Operator(OperatorDeclaration),
    // Nested type declarations
    NestedClass(ClassDeclaration),
    NestedStruct(StructDeclaration),
    NestedInterface(InterfaceDeclaration),
    NestedEnum(EnumDeclaration),
    NestedRecord(RecordDeclaration),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    /// C# 12 primary constructor parameters: struct Name(int X, int Y)
    pub primary_constructor_parameters: Option<Vec<crate::syntax::nodes::types::Parameter>>,
    pub base_types: Vec<Type>,
    pub body_declarations: Vec<StructBodyDeclaration>,
}
