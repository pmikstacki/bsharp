use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;

// Import from containing crate
use bsharp_parser::bsharp::parse_csharp_source;
use bsharp_parser::syntax::span::Span;
use bsharp_parser::expressions::statements::UsingDirective;
use bsharp_parser::parse_mode;
use bsharp_parser::syntax::node::render;
use std::sync::{Mutex, OnceLock};

/// Execute the tree command: generate a Mermaid (default) or Graphviz (DOT) visualization of the AST
pub fn execute(input: PathBuf, output: Option<PathBuf>, format: String) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    // Parse the source code using lenient parser to maximize visualization coverage.
    // Guard global parse_mode with a process-wide mutex to avoid races in parallel tests.
    static PARSE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let lock = PARSE_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().unwrap();
    let prev_strict = parse_mode::is_strict();
    parse_mode::set_strict(false);
    let parse_result = parse_csharp_source(Span::new(source_code.as_str()));
    parse_mode::set_strict(prev_strict);
    let (_remaining, ast) = parse_result.map_err(|e| anyhow!("Parse error: {:?}", e))?;

    // Determine output path based on requested format
    let fmt = format.to_lowercase();
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        match fmt.as_str() {
            "dot" | "graphviz" => path.set_extension("dot"),
            _ => path.set_extension("mmd"), // default to Mermaid extension
        };
        path
    });

    // Generate content for selected format using generic AST renderer
    let content = match fmt.as_str() {
        "dot" | "graphviz" => render::to_dot(&ast),
        _ => render::to_mermaid(&ast),
    };

    // Write the content to file (explicitly flush and sync to avoid 0-byte files)
    use std::io::Write;
    let mut file = fs::File::create(output_path.clone())
        .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;
    file.write_all(content.as_bytes())
        .with_context(|| format!("Failed to write output content: {}", output_path.display()))?;
    file.sync_all()
        .with_context(|| format!("Failed to sync output file: {}", output_path.display()))?;

    println!(
        "AST visualization (format: {}) written to: {}",
        fmt,
        output_path.display()
    );

    Ok(())
}

// Legacy helpers retained only for format_using_directive

fn format_using_directive(using: &UsingDirective) -> String {
    match using {
        UsingDirective::Namespace { namespace } => format!("using {};", namespace.to_string()),
        UsingDirective::Alias {
            alias,
            namespace_or_type,
        } => format!("using {} = {};", alias.to_string(), namespace_or_type.to_string()),
        UsingDirective::Static { type_name } => format!("using static {};", type_name.to_string()),
    }
}
