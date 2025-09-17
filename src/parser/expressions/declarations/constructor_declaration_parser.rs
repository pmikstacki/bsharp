use crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::MemberDeclaration;

/// Parse any member declaration and let the analyzer handle semantic validation
/// This is the pure structural approach - no semantic validation in syntax
pub fn parse_any_member_declaration(input: &str) -> BResult<&str, MemberDeclaration> {
    parse_member_declaration(input)
}

// TODO: Later, add parse_constructor_initializer if needed.
// fn parse_constructor_initializer(input: &str) -> BResult<&str, ConstructorInitializer> { ... }
