use syntax::span::Span;
use miette::{LabeledSpan, NamedSource, Report, SourceSpan};

use nom::IResult;
use nom_supreme::error::{BaseErrorKind, ErrorTree, StackContext};

pub type BResult<'a, O> = IResult<Span<'a>, O, ErrorTree<Span<'a>>>;

// VerboseError formatting is removed; we use ErrorTree formatting below.

/// Pretty-print a nom-supreme ErrorTree with line/column and context stack.
/// This makes test failures far more readable.
const MAX_CONTEXT_LINES: usize = 6;
const MAX_ALTERNATIVES: usize = 4;

pub fn format_error_tree(input: &str, error: &ErrorTree<Span<'_>>) -> String {
    fn line_slice(input: &str, line_no: usize) -> String {
        input
            .lines()
            .nth(line_no.saturating_sub(1))
            .unwrap_or("")
            .to_string()
    }

    fn format_stack_contexts<E: std::fmt::Display + std::fmt::Debug>(
        contexts: &[(Span<'_>, StackContext<E>)],
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

    fn max_error_offset(tree: &ErrorTree<Span<'_>>) -> usize {
        match tree {
            ErrorTree::Base { location, .. } => location.location_offset(),
            ErrorTree::Stack { base, .. } => max_error_offset(base),
            ErrorTree::Alt(list) => list.iter().map(max_error_offset).max().unwrap_or(0),
        }
    }

    fn fmt_tree(input: &str, e: &ErrorTree<Span<'_>>, acc: &mut String, depth: usize) {
        let indent = |d: usize| "  ".repeat(d);
        match e {
            ErrorTree::Base { location, kind } => {
                let _off = location.location_offset();
                let line = location.location_line() as usize;
                let col = location.get_utf8_column();
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
                    .map(|(idx, alt)| (max_error_offset(alt), idx))
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

// ===== miette integration (minimal adapter) =====
fn best_base<'a>(e: &'a ErrorTree<Span<'a>>) -> (usize, String) {
    match e {
        ErrorTree::Base { location, kind } => {
            let msg = match kind {
                BaseErrorKind::Expected(msg) => format!("expected {}", msg),
                other => format!("{:?}", other),
            };
            (location.location_offset(), msg)
        }
        ErrorTree::Stack { base, .. } => best_base(base),
        ErrorTree::Alt(list) => {
            // Choose the alternative with the greatest offset
            let mut best: Option<(usize, String)> = None;
            let mut best_off = 0usize;
            for alt in list {
                let (off, msg) = best_base(alt);
                if best.is_none() || off > best_off {
                    best = Some((off, msg));
                    best_off = off;
                }
            }
            best.unwrap_or_else(|| {
                // Fallback to a dummy at start
                (0usize, "parse error".to_string())
            })
        }
    }
}

pub fn to_miette_report<'a>(src_name: &str, src: &'a str, error: &ErrorTree<Span<'a>>) -> Report {
    let (offset, msg) = best_base(error);
    let span = SourceSpan::new((offset).into(), 1usize.into());
    let label = LabeledSpan::at(span, msg.clone());
    miette::miette!(labels = vec![label], "parse error: {}", msg)
        .with_source_code(NamedSource::new(src_name, src.to_string()))
}
