// Declare parser modules for various C# declarations

pub mod class_declaration_parser;
pub mod method_declaration_parser;
pub mod field_declaration_parser;
pub mod property_declaration_parser;
pub mod type_parameter_parser;
pub mod parameter_parser;
pub mod modifier_parser;
pub mod variable_declaration_parser;

// Add new parser modules
pub mod struct_declaration_parser;
pub mod record_declaration_parser;
pub mod interface_declaration_parser;
pub mod enum_declaration_parser;
pub mod namespace_declaration_parser;
pub mod base_types_parser;
pub mod attribute_parser;

// Common helpers modules
pub mod type_declaration_helpers;

// Re-export key node types if needed for convenience, though often handled in src/parser/nodes/mod.rs
// pub use crate::parser::nodes::declarations::TypeParameterConstraintClause;
