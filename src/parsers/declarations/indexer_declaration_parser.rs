use crate::parser::errors::{BResult, BSharpParseError, CustomErrorKind};
use crate::parser::nodes::declarations::IndexerDeclaration;

// TODO: Implement full indexer parsing
pub fn parse_indexer_declaration(input: &str) -> BResult<&str, IndexerDeclaration> {
    Err(nom::Err::Failure(BSharpParseError::new(input, CustomErrorKind::Expected("indexer_declaration_parser::parse_indexer_declaration (not implemented)"))))
} 