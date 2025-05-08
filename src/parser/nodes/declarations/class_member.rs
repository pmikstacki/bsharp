use serde::{Serialize, Deserialize};
use super::{FieldDeclaration, MethodDeclaration, EventDeclaration, IndexerDeclaration, OperatorDeclaration, ConstructorDeclaration, DestructorDeclaration, RecordDeclaration, PropertyDeclaration};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClassMember<'a> {
    Field(FieldDeclaration<'a>),
    Method(MethodDeclaration<'a>),
    Property(PropertyDeclaration<'a>),
    Event(EventDeclaration<'a>),
    Indexer(IndexerDeclaration<'a>),
    Operator(OperatorDeclaration<'a>),
    Constructor(ConstructorDeclaration<'a>),
    Destructor(DestructorDeclaration<'a>),
    Record(RecordDeclaration<'a>),
}
