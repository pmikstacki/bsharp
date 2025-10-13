use super::{
    ClassDeclaration, EnumDeclaration, InterfaceDeclaration, RecordDeclaration, StructDeclaration,
};
use super::{EventDeclaration, IndexerDeclaration, MethodDeclaration, PropertyDeclaration};
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterfaceBodyDeclaration {
    Method(MethodDeclaration),
    Property(PropertyDeclaration),
    Event(EventDeclaration),
    Indexer(IndexerDeclaration),
    // Nested types (C# 8.0+)
    NestedClass(ClassDeclaration),
    NestedStruct(StructDeclaration),
    NestedInterface(InterfaceDeclaration),
    NestedEnum(EnumDeclaration),
    NestedRecord(RecordDeclaration),
}
