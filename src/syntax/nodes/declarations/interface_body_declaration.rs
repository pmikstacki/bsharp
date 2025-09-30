use super::{EventDeclaration, IndexerDeclaration, MethodDeclaration, PropertyDeclaration};
use super::{ClassDeclaration, StructDeclaration, InterfaceDeclaration, EnumDeclaration, RecordDeclaration};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
