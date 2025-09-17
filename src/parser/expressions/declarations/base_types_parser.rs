use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::types::Type;
use crate::syntax::parser_helpers::{bchar, bseparated_list1, bws, context};
use nom::{
    combinator::{map, opt},
    sequence::preceded,
};

/// Parses a base types declaration (e.g., `: BaseType1, BaseType2`)
/// Example: `: IDisposable, IComparable<T>`
pub fn parse_base_types(input: &str) -> BResult<&str, Vec<Type>> {
    context(
        "base types declaration (expected ':' followed by comma-separated list of base types)",
        preceded(
            // Optional colon with whitespace
            opt(bws(bchar(':'))),
            // Parse comma-separated list of types (at least one if colon is present)
            map(
                opt(bseparated_list1(
                    bws(bchar(',')),
                    bws(parse_type_expression),
                )),
                |opt_types| opt_types.unwrap_or_default(),
            ),
        ),
    )(input)
}

/// Alias for compatibility with existing code
pub use parse_base_types as parse_base_type_list;

