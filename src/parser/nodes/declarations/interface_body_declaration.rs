use serde::{Serialize, Deserialize};
use super::{MethodDeclaration, PropertyDeclaration, EventDeclaration, IndexerDeclaration};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterfaceBodyDeclaration {
    Method(MethodDeclaration),
    Property(PropertyDeclaration),
    Event(EventDeclaration),
    Indexer(IndexerDeclaration),
    // TODO: Add other relevant interface members like nested types if necessary
} 