use crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::MemberDeclaration;

/// Parse any member declaration and let the analyzer handle semantic validation
/// This is the pure structural approach - no semantic validation in syntax
pub fn parse_any_member_declaration(input: &str) -> BResult<&str, MemberDeclaration> {
    parse_member_declaration(input)
}

// Constructor initializer parsing is now implemented in method_declaration_parser.rs
// as part of the unified member declaration parsing (parse_constructor_initializer)
