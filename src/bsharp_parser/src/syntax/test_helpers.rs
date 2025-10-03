use crate::parser::helpers::brace_tracker;
use crate::syntax::errors::{BResult, format_error_tree};
use nom::Err as NomErr;
use syntax::statements::statement::Statement;

/// Helper function to parse all input and ensure nothing remains
pub fn parse_all<O, P>(parser: P, input: &str) -> BResult<&str, O>
where
    P: Fn(&str) -> BResult<&str, O>,
{
    let guard = brace_tracker::install(input);
    let parse_result = parser(input);
    let status = guard.finish();

    match parse_result {
        Ok((remaining, result)) => {
            if remaining.trim().is_empty() {
                // Expose final status to allow callers (like CLI) to inspect unmatched braces.
                brace_tracker::store_status(status);
                Ok(("", result))
            } else {
                // Prefer unmatched brace diagnostic if available
                use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
                let error_tree = if let Some(offset) = status.unmatched_open {
                    let location = &input[offset..];
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
pub fn parse_statement_all(input: &str) -> BResult<&str, Statement> {
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
