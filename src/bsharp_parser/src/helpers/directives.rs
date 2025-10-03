use crate::syntax::comment_parser::ws;
use crate::trivia::preprocessor_directive_parser::parse_preprocessor_directive;

/// Skip preprocessor directives starting at the current position.
/// If `ws_first` is true, consumes whitespace/comments before scanning directives.
/// Returns the remaining slice after skipping zero or more directives and trailing whitespace.
pub fn skip_preprocessor_directives(mut input: &str, ws_first: bool) -> &str {
    if ws_first {
        if let Ok((r, _)) = ws(input) {
            input = r;
        }
    }
    loop {
        // Always consume whitespace/comments between attempts
        if let Ok((r, _)) = ws(input) {
            input = r;
        }
        match parse_preprocessor_directive(input) {
            Ok((rest, _dir)) => {
                input = rest;
                // Continue loop to consume subsequent directives
            }
            Err(_) => break,
        }
    }
    input
}
