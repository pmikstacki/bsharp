use super::class_declaration::ClassDeclaration;
use super::delegate_declaration::DelegateDeclaration;
use super::enum_declaration::EnumDeclaration;
use super::interface_declaration::InterfaceDeclaration;
use super::record_declaration::RecordDeclaration;
use super::struct_declaration::StructDeclaration;
use serde::{Deserialize, Serialize};

/// Enum representing any type of top-level or nested declaration that defines a type.
#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeDeclaration {
    Class(ClassDeclaration),
    Struct(StructDeclaration),
    Record(RecordDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Delegate(DelegateDeclaration),
}
