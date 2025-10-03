use super::attribute::AttributeList;
use super::constructor_declaration::ConstructorDeclaration;
use super::event_declaration::EventDeclaration;
use super::field_declaration::FieldDeclaration;
use super::indexer_declaration::IndexerDeclaration;
use super::method_declaration::MethodDeclaration;
use super::modifier::Modifier;
use super::operator_declaration::OperatorDeclaration;
use super::property_declaration::PropertyDeclaration;
use super::{ClassDeclaration, EnumDeclaration, InterfaceDeclaration, RecordDeclaration};
use crate::Identifier;
use crate::types::{Parameter, Type, TypeParameter};
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
    pub primary_constructor_parameters: Option<Vec<Parameter>>,
    pub base_types: Vec<Type>,
    pub body_declarations: Vec<StructBodyDeclaration>,
}
