use crate::syntax::errors::BResult;
use nom::Err;

/// Helper function to parse all input and ensure nothing remains
pub fn parse_all<'a, O, P>(parser: P, input: &'a str) -> BResult<&'a str, O>
where
    P: Fn(&'a str) -> BResult<&'a str, O>,
{
    let (remaining, result) = parser(input)?;
    if remaining.trim().is_empty() {
        Ok(("", result))
    } else {
        // Create a nom-supreme error for remaining input
        use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
        let error_tree = ErrorTree::Base {
            location: remaining,
            kind: BaseErrorKind::Expected(Expectation::Eof),
        };
        Err(Err::Error(error_tree))
    }
}

/// Parse statement and ensure all input is consumed
pub fn parse_statement_all(input: &str) -> BResult<&str, crate::syntax::nodes::statements::statement::Statement> {
    parse_all(crate::parser::statement_parser::parse_statement, input)
}

/// Helper function to unwrap a syntax result or panic on error.
/// Used in tests to simplify result handling.
pub fn parse_input_unwrap<T>(parser_result: BResult<&str, T>) -> (&str, T) {
    match parser_result {
        Ok(result) => result,
        Err(e) => panic!("Parse error: {:?}", e),
    }
}

// If parse_input_unwrap is also intended to be here, it can be added.
// For now, focusing on making parse_all and parse_all_statement available.
