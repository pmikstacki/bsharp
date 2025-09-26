use nom::error::{convert_error, VerboseError};
use nom::IResult;
use nom_supreme::error::{BaseErrorKind, ErrorTree, StackContext};

pub type BResult<I, O> = IResult<I, O, ErrorTree<I>>;

/// Helper function to format VerboseError with line/column information
pub fn format_parse_error(input: &str, error: VerboseError<&str>) -> String {
    convert_error(input, error)
}

/// Pretty-print a nom-supreme ErrorTree with line/column and context stack.
/// This makes test failures far more readable.
pub fn format_error_tree(input: &str, error: &ErrorTree<&str>) -> String {
    fn byte_offset(input: &str, location: &str) -> usize {
        let ip = input.as_ptr() as usize;
        let lp = location.as_ptr() as usize;
        lp.saturating_sub(ip)
    }

    fn line_col(input: &str, offset: usize) -> (usize, usize) {
        let mut line = 1usize;
        let mut col = 1usize;
        for (i, ch) in input.char_indices() {
            if i >= offset { break; }
            if ch == '\n' { line += 1; col = 1; } else { col += 1; }
        }
        (line, col)
    }

    fn line_slice(input: &str, line_no: usize) -> String {
        input.lines().nth(line_no.saturating_sub(1)).unwrap_or("").to_string()
    }

    fn format_stack_contexts<E: std::fmt::Display + std::fmt::Debug>(
        contexts: &[(&str, StackContext<E>)],
    ) -> String {
        if contexts.is_empty() { return String::new(); }
        let mut out = String::new();
        out.push_str("Contexts:\n");
        for (_, ctx) in contexts {
            match ctx {
                StackContext::Context(name) => out.push_str(&format!("  - {}\n", name)),
                StackContext::Kind(kind) => out.push_str(&format!("  - {:?}\n", kind)),
            }
        }
        out
    }

    fn fmt_tree(input: &str, e: &ErrorTree<&str>, acc: &mut String, depth: usize) {
        let indent = |d: usize| "  ".repeat(d);
        match e {
            ErrorTree::Base { location, kind } => {
                let off = byte_offset(input, location);
                let (line, col) = line_col(input, off);
                let src_line = line_slice(input, line);
                let kind_msg = match kind {
                    BaseErrorKind::Expected(msg) => format!("expected {}", msg),
                    // Handle external errors and any other diagnostics generically
                    other => format!("{:?}", other),
                };
                acc.push_str(&format!(
                    "{}at {}:{}: {}\n{}{}\n{}{}^\n",
                    indent(depth),
                    line,
                    col,
                    kind_msg,
                    indent(depth),
                    src_line,
                    indent(depth),
                    " ".repeat(col.saturating_sub(1)),
                ));
            }
            ErrorTree::Stack { base, contexts } => {
                // Print base, then contexts
                fmt_tree(input, base, acc, depth);
                acc.push_str(&format!("{}{}", indent(depth), format_stack_contexts(contexts)));
            }
            ErrorTree::Alt(list) => {
                acc.push_str(&format!("{}alternatives ({}):\n", indent(depth), list.len()));
                for (i, alt) in list.iter().enumerate() {
                    acc.push_str(&format!("{}- alt #{}:\n", indent(depth), i + 1));
                    fmt_tree(input, alt, acc, depth + 1);
                }
            }
        }
    }

    let mut out = String::new();
    fmt_tree(input, error, &mut out, 0);
    out
}
