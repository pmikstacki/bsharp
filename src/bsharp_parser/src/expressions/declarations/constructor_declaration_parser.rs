use crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration;
use crate::syntax::errors::BResult;
use syntax::declarations::MemberDeclaration;

/// Parse any member declaration and let the analyzer handle semantic validation
/// This is the pure structural approach - no semantic validation in syntax
pub fn parse_any_member_declaration<'a>(input: Span<'a>) -> BResult<'a, MemberDeclaration> {
    parse_member_declaration(input)
}

// Constructor initializer parsing is now implemented in method_declaration_parser.rs
// as part of the unified member declaration parsing (parse_constructor_initializer)
use crate::syntax::span::Span;
