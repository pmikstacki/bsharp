use crate::syntax::errors::{format_error_tree, BResult};
use nom::Err as NomErr;

/// Helper function to parse all input and ensure nothing remains
pub fn parse_all<O, P>(parser: P, input: &str) -> BResult<&str, O>
where
    P: Fn(&str) -> BResult<&str, O>,
{
    let (remaining, result) = parser(input)?;
    if remaining.trim().is_empty() {
        Ok(("", result))
    } else {
        // Create a nom-supreme error for remaining input
        use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
        let error_tree = ErrorTree::Base {
            location: remaining,
            kind: BaseErrorKind::Expected(Expectation::Eof),
        };
        Err(NomErr::Error(error_tree))
    }
}

/// Parse statement and ensure all input is consumed
pub fn parse_statement_all(
    input: &str,
) -> BResult<&str, crate::syntax::nodes::statements::statement::Statement> {
    parse_all(crate::parser::statement_parser::parse_statement, input)
}

/// Helper function to unwrap a syntax result or panic on error.
/// Used in tests to simplify result handling.
pub fn parse_input_unwrap<T>(parser_result: BResult<&str, T>) -> (&str, T) {
    match parser_result {
        Ok(result) => result,
        Err(e) => {
            // We don't have the original input here, so this helper can't pretty-print.
            // Prefer using `expect_ok(input, res)` below where possible.
            panic!("Parse error: {:?}", e)
        }
    }
}

/// Expect a parsing result to be Ok; on error, pretty-print with line/column and context stack.
pub fn expect_ok<'a, T>(input: &'a str, res: BResult<&'a str, T>) -> (&'a str, T) {
    match res {
        Ok(ok) => ok,
        Err(NomErr::Error(tree)) | Err(NomErr::Failure(tree)) => {
            panic!("{}", format_error_tree(input, &tree))
        }
        Err(NomErr::Incomplete(_)) => panic!("Parse requires more input"),
    }
}

// If parse_input_unwrap is also intended to be here, it can be added.
// For now, focusing on making parse_all and parse_all_statement available.
