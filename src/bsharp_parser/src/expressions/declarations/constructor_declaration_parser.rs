use crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration;
use crate::syntax::errors::BResult;
use crate::syntax::span::Span;
use syntax::declarations::MemberDeclaration;

/// Parse any member declaration and let the analyzer handle semantic validation
/// This is the pure structural approach - no semantic validation in syntax
pub fn parse_any_member_declaration(input: Span) -> BResult<MemberDeclaration> {
    parse_member_declaration(input)
}