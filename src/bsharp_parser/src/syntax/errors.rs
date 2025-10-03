use nom::IResult;
use nom::error::{VerboseError, convert_error};
use nom_supreme::error::{BaseErrorKind, ErrorTree, StackContext};

pub type BResult<I, O> = IResult<I, O, ErrorTree<I>>;

/// Helper function to format VerboseError with line/column information
pub fn format_parse_error(input: &str, error: VerboseError<&str>) -> String {
    convert_error(input, error)
}

/// Pretty-print a nom-supreme ErrorTree with line/column and context stack.
/// This makes test failures far more readable.
const MAX_CONTEXT_LINES: usize = 6;
const MAX_ALTERNATIVES: usize = 4;

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
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }

    fn line_slice(input: &str, line_no: usize) -> String {
        input
            .lines()
            .nth(line_no.saturating_sub(1))
            .unwrap_or("")
            .to_string()
    }

    fn format_stack_contexts<E: std::fmt::Display + std::fmt::Debug>(
        contexts: &[(&str, StackContext<E>)],
    ) -> String {
        if contexts.is_empty() {
            return String::new();
        }
        let total = contexts.len();
        let start = total.saturating_sub(MAX_CONTEXT_LINES);
        let skipped = start;

        let mut out = String::new();
        out.push_str("Contexts:\n");
        if skipped > 0 {
            out.push_str(&format!("  - … ({} more context frames)\n", skipped));
        }

        let mut last_label: Option<String> = None;
        for (_, ctx) in contexts.iter().skip(start) {
            let label = match ctx {
                StackContext::Context(name) => name.to_string(),
                StackContext::Kind(kind) => format!("{:?}", kind),
            };
            if last_label.as_ref() == Some(&label) {
                continue;
            }
            out.push_str(&format!("  - {}\n", label));
            last_label = Some(label);
        }
        out
    }

    fn max_error_offset<'a>(input: &'a str, tree: &ErrorTree<&'a str>) -> usize {
        match tree {
            ErrorTree::Base { location, .. } => byte_offset(input, location),
            ErrorTree::Stack { base, .. } => max_error_offset(input, base),
            ErrorTree::Alt(list) => list
                .iter()
                .map(|alt| max_error_offset(input, alt))
                .max()
                .unwrap_or(0),
        }
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
                acc.push_str(&format!(
                    "{}{}",
                    indent(depth),
                    format_stack_contexts(contexts)
                ));
            }
            ErrorTree::Alt(list) => {
                if list.is_empty() {
                    return;
                }

                let scored: Vec<(usize, usize)> = list
                    .iter()
                    .enumerate()
                    .map(|(idx, alt)| (max_error_offset(input, alt), idx))
                    .collect();

                let best_offset = scored.iter().map(|(offset, _)| *offset).max().unwrap_or(0);

                let mut selected: Vec<usize> = scored
                    .into_iter()
                    .filter(|(offset, _)| *offset == best_offset)
                    .map(|(_, idx)| idx)
                    .collect();

                let truncated = selected.len().saturating_sub(MAX_ALTERNATIVES);
                if selected.len() > MAX_ALTERNATIVES {
                    selected.truncate(MAX_ALTERNATIVES);
                }

                acc.push_str(&format!(
                    "{}alternatives (showing {} of {}, best error at byte {}):\n",
                    indent(depth),
                    selected.len(),
                    list.len(),
                    best_offset,
                ));

                for idx in selected {
                    acc.push_str(&format!("{}- alt #{}:\n", indent(depth), idx + 1));
                    fmt_tree(input, &list[idx], acc, depth + 1);
                }

                if truncated > 0 {
                    acc.push_str(&format!(
                        "{}- … ({} more equally-ranked alternatives omitted)\n",
                        indent(depth),
                        truncated,
                    ));
                }
            }
        }
    }

    let mut out = String::new();
    fmt_tree(input, error, &mut out, 0);
    out
}
