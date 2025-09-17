use anyhow::{anyhow, Context, Result};
use serde_json::to_string_pretty;
use std::fs;
use std::path::PathBuf;

// Import the syntax from the containing crate
use crate::syntax;

/// Execute the parse command: parse C# file and output JSON
pub fn execute(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    // Parse the source code
    let parser = syntax::Parser::new();
    let ast = parser
        .parse(&source_code)
        .map_err(|e| anyhow!("Parse error: {}", e))?;

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
