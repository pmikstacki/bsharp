use anyhow::{Context, Result};
use serde_json::to_string_pretty;
use std::fs;
use std::path::PathBuf;

// Import the syntax from the containing crate
use crate::parser::bsharp::parse_csharp_source;
use nom::Finish;
use crate::syntax::errors::format_error_tree;
use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
use std::env;

/// Execute the parse command: parse C# file and output JSON
/// On parse failure, pretty-print errors by default and exit non-zero.
/// If `errors_json` is true, emit a JSON error object to stdout instead.
/// If `no_color` is true (or NO_COLOR is set), ANSI colors are disabled in pretty output.
pub fn execute(input: PathBuf, output: Option<PathBuf>, errors_json: bool, no_color: bool) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    // Parse the source code (low-level) to preserve structured errors
    let (remaining, ast) = match parse_csharp_source(&source_code).finish() {
        Ok(ok) => ok,
        Err(e) => {
            let pretty = format_error_tree(&source_code, &e);
            if errors_json {
                let payload = serde_json::json!({
                    "error": { "kind": "parse_error", "file": input.display().to_string(), "message": pretty }
                });
                println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
            } else {
                print_pretty_error(&input, &pretty, no_color);
            }
            std::process::exit(1);
        }
    };

    // Treat significant trailing input as a pretty error with location and caret
    if !remaining.trim().is_empty() {
        // Build a synthetic EOF-expected error at the remaining location
        let tree = ErrorTree::Base { location: remaining, kind: BaseErrorKind::Expected(Expectation::Eof) };
        let pretty = format_error_tree(&source_code, &tree);
        if errors_json {
            let payload = serde_json::json!({
                "error": { "kind": "trailing_input", "file": input.display().to_string(), "message": pretty }
            });
            println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
        } else {
            print_pretty_error(&input, &pretty, no_color);
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
