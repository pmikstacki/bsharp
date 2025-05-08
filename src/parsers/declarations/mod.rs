// Declare parser modules for various C# declarations

pub mod class_declaration_parser;
pub mod method_declaration_parser;
pub mod field_declaration_parser;
pub mod property_declaration_parser;
pub mod type_parameter_parser;
pub mod parameter_parser;
pub mod modifier_parser;
// Add the missing module declaration
pub mod variable_declaration_parser;

// Re-export key node types if needed for convenience, though often handled in src/parser/nodes/mod.rs
// pub use crate::parser::nodes::declarations::TypeParameterConstraintClause;
