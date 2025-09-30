use anyhow::{Context, Result};
use serde_json::to_string_pretty;
use std::fs;
use std::path::PathBuf;

// Import the syntax from the containing crate
use crate::syntax;

/// Execute the parse command: parse C# file and output JSON
/// On parse failure, pretty-print errors by default and exit non-zero.
/// If `errors_json` is true, emit a JSON error object to stdout instead.
pub fn execute(input: PathBuf, output: Option<PathBuf>, errors_json: bool) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    // Parse the source code
    let parser = syntax::Parser::new();
    let ast = match parser.parse(&source_code) {
        Ok(ast) => ast,
        Err(err_msg) => {
            if errors_json {
                // Emit machine-readable JSON error to stdout
                let payload = serde_json::json!({
                    "error": {
                        "kind": "parse_error",
                        "file": input.display().to_string(),
                        "message": err_msg,
                    }
                });
                println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
            } else {
                // Pretty-print error to stderr
                eprintln!(
                    "error: failed to parse file\n  --> {}\n   = {}",
                    input.display(),
                    err_msg
                );
            }
            // Ensure non-zero exit without writing any files
            std::process::exit(1);
        }
    };

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
