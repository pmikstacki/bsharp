use super::{ClassDeclaration, EnumDeclaration, InterfaceDeclaration, StructDeclaration};
use super::{
    ConstructorDeclaration, DestructorDeclaration, EventDeclaration, FieldDeclaration,
    IndexerDeclaration, MethodDeclaration, OperatorDeclaration, PropertyDeclaration,
    RecordDeclaration,
};
use bsharp_syntax_derive::AstNode;
use serde::{Deserialize, Serialize};

#[derive(AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
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
