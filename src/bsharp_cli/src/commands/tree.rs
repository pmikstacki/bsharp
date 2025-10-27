use anyhow::{Context, Result, anyhow};
use clap::{Args, arg};
use std::fs;
use std::path::PathBuf;

// Import from containing crate
use bsharp_parser::bsharp::parse_csharp_source;
use bsharp_parser::parse_mode;
use bsharp_parser::syntax::node::render;
use bsharp_syntax::span::Span;
use std::sync::{Mutex, OnceLock};

#[derive(Args, Debug, Clone)]
pub struct TreeArgs {
    /// The input C# file to parse
    #[arg(required = true)]
    pub input: PathBuf,

    /// The output file (defaults to <input>.mmd for Mermaid or <input>.dot for Graphviz)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Output format: mermaid (default) or dot/graphviz
    #[arg(long, value_parser = ["mermaid", "dot", "graphviz"], default_value = "mermaid")]
    pub format: String,
}

/// Execute the tree command: generate a Mermaid (default) or Graphviz (DOT) visualization of the AST
pub fn execute(args: TreeArgs) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read file: {}", args.input.display()))?;

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
    let fmt = args.format.to_lowercase();
    let output_path = args.output.clone().unwrap_or_else(|| {
        let mut path = args.input.clone();
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

// (removed unused legacy helper)
