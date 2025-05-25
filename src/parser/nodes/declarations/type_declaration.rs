use serde::{Serialize, Deserialize};
use super::class_declaration::ClassDeclaration;
use super::struct_declaration::StructDeclaration;
use super::record_declaration::RecordDeclaration;
use super::interface_declaration::InterfaceDeclaration;
use super::enum_declaration::EnumDeclaration;
use super::delegate_declaration::DelegateDeclaration;

/// Enum representing any type of top-level or nested declaration that defines a type.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeDeclaration {
    Class(ClassDeclaration),
    Struct(StructDeclaration),
    Record(RecordDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Delegate(DelegateDeclaration),
}
