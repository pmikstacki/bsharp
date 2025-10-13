use crate::parser::helpers::brace_tracker;
use crate::syntax::errors::{format_error_tree, BResult};
use nom::Err as NomErr;
use syntax::statements::statement::Statement;

/// Helper function to parse all input and ensure nothing remains
pub fn parse_all<'a, O, P>(mut parser: P, input: Span<'a>) -> BResult<'a, O>
where
    P: FnMut(Span<'a>) -> BResult<'a, O>,
{
    let guard = brace_tracker::install(input);
    let parse_result = parser(input);
    let status = guard.finish();

    match parse_result {
        Ok((remaining, result)) => {
            if remaining.fragment().trim().is_empty() {
                // Expose final status to allow callers (like CLI) to inspect unmatched braces.
                brace_tracker::store_status(status);
                Ok((remaining, result))
            } else {
                // Prefer unmatched brace diagnostic if available
                use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
                let error_tree = if let Some(offset) = status.unmatched_open {
                    let location = remaining; // point at remaining for clarity
                    ErrorTree::Base {
                        location,
                        kind: BaseErrorKind::Expected(Expectation::Char('}')),
                    }
                } else {
                    ErrorTree::Base {
                        location: remaining,
                        kind: BaseErrorKind::Expected(Expectation::Eof),
                    }
                };
                let err = NomErr::Error(error_tree);
                brace_tracker::store_status(status);
                Err(err)
            }
        }
        Err(err) => {
            brace_tracker::store_status(status);
            Err(err)
        }
    }
}

/// Parse statement and ensure all input is consumed
pub fn parse_statement_all<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    parse_all(crate::parser::statement_parser::parse_statement, input)
}

/// Helper function to unwrap a syntax result or panic on error.
/// Used in tests to simplify result handling.
pub fn parse_input_unwrap<'a, T>(parser_result: BResult<'a, T>) -> (Span<'a>, T) {
    parser_result.unwrap_or_else(|e| panic!("Parse error: {:?}", e))
}

/// Expect a parsing result to be Ok; on error, pretty-print with line/column and context stack.
pub fn expect_ok<'a, T>(input: &'a str, res: BResult<'a, T>) -> (Span<'a>, T) {
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
use crate::syntax::span::Span;
