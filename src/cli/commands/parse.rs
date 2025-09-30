use anyhow::{Context, Result};
use serde_json::to_string_pretty;
use std::fs;
use std::path::PathBuf;

// Import the syntax from the containing crate
use crate::parser::bsharp::{parse_csharp_source, parse_csharp_source_strict};
use crate::parser::parse_mode;
use nom::Finish;
use crate::syntax::errors::format_error_tree;
use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
use std::env;
/// Execute the parse command: parse C# file and output JSON
/// On parse failure, pretty-print errors by default and exit non-zero.
/// If `errors_json` is true, emit a JSON error object to stdout instead.
/// If `no_color` is true (or NO_COLOR is set), ANSI colors are disabled in pretty output.
pub fn execute(input: PathBuf, output: Option<PathBuf>, errors_json: bool, no_color: bool, lenient: bool) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    // Select strict or lenient parser and set strict flag for deep parsers
    let parser = if lenient { parse_csharp_source } else { parse_csharp_source_strict };
    let prev_strict = parse_mode::is_strict();
    parse_mode::set_strict(!lenient);

    // Parse the source code (low-level) to preserve structured errors
    let parse_result = parser(&source_code).finish();
    // Restore previous strict mode to avoid leaking state
    parse_mode::set_strict(prev_strict);
    let (remaining, ast) = match parse_result {
        Ok(ok) => ok,
        Err(e) => {
            let pretty = pretty_deepest_error(&source_code, &e);
            if errors_json {
                let (line, column, line_text) = error_location_summary(&source_code, &e);
                let (expected, found) = expected_found_summary(&e);
                let payload = serde_json::json!({
                    "error": {
                        "kind": "parse_error",
                        "file": input.display().to_string(),
                        "line": line,
                        "column": column,
                        "expected": expected,
                        "found": found,
                        "line_text": line_text,
                        "message": pretty
                    }
                });
                println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
            } else {
                print_pretty_error(&input, &pretty, no_color);
            }
            std::process::exit(1);
        }
    };

    // Treat significant trailing input as a pretty error with location and caret
    if !lenient && !remaining.trim().is_empty() {
        if let Some((_off, line, col, line_text)) = find_unmatched_open_brace(&source_code) {
            let mut pretty = String::new();
            pretty.push_str(&format!("at {}:{}: expected '}}' before end of file\n", line, col));
            pretty.push_str(&line_text);
            pretty.push('\n');
            pretty.push_str(&" ".repeat(col.saturating_sub(1)));
            pretty.push('^');

            if errors_json {
                let payload = serde_json::json!({
                    "error": {
                        "kind": "unmatched_brace",
                        "file": input.display().to_string(),
                        "line": line,
                        "column": col,
                        "expected": "}",
                        "found": "<eof>",
                        "line_text": line_text,
                        "message": pretty
                    }
                });
                println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
            } else {
                print_pretty_error(&input, &pretty, no_color);
            }
        } else {
            // Build a synthetic EOF-expected error at the remaining location
            let tree = ErrorTree::Base { location: remaining, kind: BaseErrorKind::Expected(Expectation::Eof) };
            let pretty = format_error_tree(&source_code, &tree);
            if errors_json {
                let (line, column, line_text) = error_location_summary(&source_code, &tree);
                let (expected, found) = expected_found_summary(&tree);
                let payload = serde_json::json!({
                    "error": { "kind": "trailing_input", "file": input.display().to_string(),
                        "line": line,
                        "column": column,
                        "expected": expected,
                        "found": found,
                        "line_text": line_text,
                        "message": pretty
                    }
                });
                println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
            } else {
                print_pretty_error(&input, &pretty, no_color);
            }
        }
        std::process::exit(1);
    }

    // Serialize the AST to JSON
    let json = to_string_pretty(&ast).context("Failed to serialize AST to JSON")?;

    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension("json");
        path
    });

    // Write the JSON to file
    fs::write(&output_path, json)
        .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;

    println!("JSON parse tree written to: {}", output_path.display());

    Ok(())
}

fn print_pretty_error(input: &PathBuf, pretty_body: &str, no_color_flag: bool) {
    let no_color_env = env::var("NO_COLOR").is_ok();
    let colored = !(no_color_flag || no_color_env);

    let (err_hdr, arrow) = if colored { ("\x1b[1;31merror\x1b[0m:", "\x1b[31m^\x1b[0m") } else { ("error:", "^") };
    let file = if colored { format!("\x1b[36m{}\x1b[0m", input.display()) } else { format!("{}", input.display()) };

    // pretty_body already includes line/col and caret; optionally we can recolor caret by replacing '^'
    // For simplicity, prepend header and file path; leave body as-is.
    eprintln!("{} failed to parse file\n  --> {}\n{}", err_hdr, file, pretty_body.replace('^', arrow));
}

// Best-effort unmatched '{' detector. Handles //, /* */ comments and basic string/char literals.
fn find_unmatched_open_brace(src: &str) -> Option<(usize, usize, usize, String)> {
    #[derive(PartialEq)]
    enum State { Normal, LineComment, BlockComment, String, Char }
    let mut state = State::Normal;
    let mut stack: Vec<usize> = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        match state {
            State::Normal => {
                if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i+1] == b'/' { state = State::LineComment; i += 2; continue; }
                if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i+1] == b'*' { state = State::BlockComment; i += 2; continue; }
                if bytes[i] == b'"' { state = State::String; i += 1; continue; }
                if bytes[i] == b'\'' { state = State::Char; i += 1; continue; }
                if bytes[i] == b'{' { stack.push(i); }
                if bytes[i] == b'}' { stack.pop(); }
                i += 1;
            }
            State::LineComment => { if bytes[i] == b'\n' { state = State::Normal; } i += 1; }
            State::BlockComment => { if i+1 < bytes.len() && bytes[i] == b'*' && bytes[i+1] == b'/' { state = State::Normal; i += 2; } else { i += 1; } }
            State::String => {
                if bytes[i] == b'\\' { i += 2; continue; }
                if bytes[i] == b'"' { state = State::Normal; }
                i += 1;
            }
            State::Char => {
                if bytes[i] == b'\\' { i += 2; continue; }
                if bytes[i] == b'\'' { state = State::Normal; }
                i += 1;
            }
        }
    }
    let off = *stack.last()?;
    // Compute line/col and line text
    let mut line = 1usize; let mut col = 1usize;
    for (pos, ch) in src.char_indices() {
        if pos >= off { break; }
        if ch == '\n' { line += 1; col = 1; } else { col += 1; }
    }
    let line_text = src.lines().nth(line.saturating_sub(1)).unwrap_or("").to_string();
    Some((off, line, col, line_text))
}

// Extract a representative location from an ErrorTree for JSON enrichment
fn error_location_summary<'a>(input: &'a str, err: &nom_supreme::error::ErrorTree<&'a str>) -> (usize, usize, String) {
    use nom_supreme::error::ErrorTree;
    fn byte_offset(input: &str, location: &str) -> usize {
        let ip = input.as_ptr() as usize;
        let lp = location.as_ptr() as usize;
        lp.saturating_sub(ip)
    }
    fn line_col(input: &str, offset: usize) -> (usize, usize) {
        let mut line = 1usize; let mut col = 1usize;
        for (i, ch) in input.char_indices() {
            if i >= offset { break; }
            if ch == '\n' { line += 1; col = 1; } else { col += 1; }
        }
        (line, col)
    }
    fn line_slice(input: &str, line_no: usize) -> String {
        input.lines().nth(line_no.saturating_sub(1)).unwrap_or("").to_string()
    }
    // Walk the tree and select the Base with the maximum byte offset (deepest location)
    fn deepest_base<'a>(input: &'a str, e: &'a ErrorTree<&'a str>) -> (&'a str, usize) {
        match e {
            ErrorTree::Base { location, .. } => (*location, byte_offset(input, location)),
            ErrorTree::Stack { base, .. } => deepest_base(input, base),
            ErrorTree::Alt(list) => {
                let mut best_loc: Option<(&str, usize)> = None;
                for child in list {
                    let cand = deepest_base(input, child);
                    if best_loc.map(|(_, off)| cand.1 > off).unwrap_or(true) {
                        best_loc = Some(cand);
                    }
                }
                best_loc.unwrap_or((input, 0))
            }
        }
    }
    let (_loc, off) = deepest_base(input, err);
    let (line, col) = line_col(input, off);
    (line, col, line_slice(input, line))
}

fn expected_found_summary<'a>(err: &nom_supreme::error::ErrorTree<&'a str>) -> (String, String) {
    use nom_supreme::error::{BaseErrorKind, ErrorTree};
    match err {
        ErrorTree::Base { kind, .. } => match kind {
            BaseErrorKind::Expected(exp) => (format!("{}", exp), String::new()),
            other => (format!("{:?}", other), String::new()),
        },
        ErrorTree::Stack { base, .. } => expected_found_summary(base),
        ErrorTree::Alt(list) => {
            if let Some(first) = list.first() { expected_found_summary(first) } else { (String::new(), String::new()) }
        }
    }
}
