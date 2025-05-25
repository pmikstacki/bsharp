use crate::parser::errors::{BResult, BSharpParseError, CustomErrorKind};
use crate::parser::nodes::declarations::EventDeclaration;

// TODO: Implement full event parsing
pub fn parse_event_declaration(input: &str) -> BResult<&str, EventDeclaration> {
    // For now, return an error or unimplemented
    Err(nom::Err::Failure(BSharpParseError::new(input, CustomErrorKind::Expected("event_declaration_parser::parse_event_declaration (not implemented)"))))
} 