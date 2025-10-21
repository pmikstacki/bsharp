use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::span::Span;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::types::Type;

/// Parses a base types declaration (e.g., `: BaseType1, BaseType2`)
/// Example: `: IDisposable, IComparable<T>`
pub fn parse_base_types(input: Span) -> BResult<Vec<Type>> {
    preceded(
        // Optional colon with whitespace
        opt(delimited(ws, tok_colon(), ws)),
        // Parse comma-separated list of types (at least one if colon is present)
        opt(separated_list1(
            |i| delimited(ws, tok_comma(), ws).parse(i),
            |i| delimited(ws, parse_type_expression, ws).parse(i),
        ))
            .map(|opt_types| opt_types.unwrap_or_default())
            .context("base types"),
    )
        .parse(input)
}

/// Alias for compatibility with existing code
pub use parse_base_types as parse_base_type_list;
use crate::tokens::separators::{tok_colon, tok_comma};


