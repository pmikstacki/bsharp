use serde::{Serialize, Deserialize};
use super::{FieldDeclaration, MethodDeclaration, EventDeclaration, IndexerDeclaration, OperatorDeclaration, ConstructorDeclaration, DestructorDeclaration, RecordDeclaration, PropertyDeclaration};
use super::{ClassDeclaration, StructDeclaration, InterfaceDeclaration, EnumDeclaration};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClassBodyDeclaration {
    Field(FieldDeclaration),
    Method(MethodDeclaration),
    Property(PropertyDeclaration),
    Event(EventDeclaration),
    Indexer(IndexerDeclaration),
    Operator(OperatorDeclaration),
    Constructor(ConstructorDeclaration),
    Destructor(DestructorDeclaration),
    Record(RecordDeclaration),
    // Nested type declarations
    NestedClass(ClassDeclaration),
    NestedStruct(StructDeclaration),
    NestedInterface(InterfaceDeclaration),
    NestedEnum(EnumDeclaration),
    NestedRecord(RecordDeclaration),
} 