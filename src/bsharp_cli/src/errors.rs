use bsharp_parser::errors as perr;
use bsharp_syntax::span::Span;
use nom_supreme::error::ErrorTree;

use crate::commands::parse::ParseArgs;

fn deepest_span<'a>(e: &'a ErrorTree<Span<'a>>) -> Span<'a> {
    match e {
        ErrorTree::Base { location, .. } => *location,
        ErrorTree::Stack { base, .. } => deepest_span(base),
        ErrorTree::Alt(list) => list
            .iter()
            .map(|child| deepest_span(child))
            .max_by_key(|s| s.location_offset())
            .unwrap_or_else(|| Span::new("")),
    }
}

pub fn emit_json_error(args: &ParseArgs, source_code: &str, err_tree: &ErrorTree<Span>) {
    let loc = deepest_span(err_tree);
    let line = loc.location_line() as usize;
    let column = loc.get_utf8_column();
    let line_text = source_code
        .lines()
        .nth(line.saturating_sub(1))
        .unwrap_or("")
        .to_string();

    let (expected, found) = (String::new(), String::new());
    let mut error_obj = serde_json::json!({
        "kind": "parse_error",
        "file": args.input.display().to_string(),
        "line": line,
        "column": column,
        "expected": expected,
        "found": found,
        "line_text": line_text,
        "message": perr::format_error_tree(source_code, err_tree),
    });

    if args.emit_spans {
        let offset = loc.location_offset();
        error_obj["spans"] = serde_json::json!({
            "abs": {"start": offset, "end": offset + 1},
            "rel": {"start": {"line": line, "column": column}, "end": {"line": line, "column": column + 1}}
        });
    }

    let payload = serde_json::json!({"error": error_obj});
    println!(
        "{}",
        serde_json::to_string(&payload).unwrap_or_else(|_| {
            "{\"error\":{\"message\":\"parse error\"}}".to_string()
        })
    );
}

pub fn print_pretty_error(args: &ParseArgs, source_code: &str, err_tree: &ErrorTree<Span>) {
    let report = perr::to_miette_report(&args.input.display().to_string(), source_code, err_tree);
    eprintln!("{:?}", report);
}
