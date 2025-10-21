mod tests_writer;
mod generator;

use clap::{Parser, ArgAction};
use std::path::PathBuf;
use tests_writer::utility;

/// Generate Rust parser compliance tests from Roslyn C# syntax tests.
#[derive(Debug, Parser)]
#[command(name = "bsharp-compliance" )]
#[command(about = "Generates Rust tests from Roslyn C# syntax tests", long_about = None)]
struct Args {
    /// Source directory with Roslyn Parsing tests
    #[arg(long, default_value = "roslyn_repo/src/Compilers/CSharp/Test/Syntax/Parsing")] 
    src: PathBuf,

    /// Destination directory for generated Rust tests
    #[arg(long, default_value = "../bsharp_compliance_testing/src/generated")] 
    dst: PathBuf,

    /// Include glob patterns (repeatable)
    #[arg(long)]
    include: Vec<String>,

    /// Exclude glob patterns (repeatable)
    #[arg(long)]
    exclude: Vec<String>,

    /// Maximum tests to generate per source file
    #[arg(long, default_value_t = 200usize)]
    max_per_file: usize,

    /// Skip classes overriding ParseNode/ParseTree
    #[arg(long, default_value_t = false)]
    skip_overrides: bool,

    /// Skip tests asserting diagnostics (expectedErrors)
    #[arg(long, default_value_t = false)]
    skip_diagnostics: bool,

    /// Skip parsing Roslyn TestOptions.* flags (new generator only)
    #[arg(long, default_value_t = false)]
    skip_options: bool,

    /// Verbose logging during generation
    #[arg(long, default_value_t = false)]
    verbose: bool,

    /// Dry-run without writing files
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Stop on first error
    #[arg(long, default_value_t = false)]
    fail_fast: bool,

    /// Use the new emitter+writer pipeline (experimental)
    #[arg(long, default_value_t = false)]
    new_emitter: bool,

    /// Prevalidate cases with our parser before generating (pass --prevalidate=false to include failing parses)
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    prevalidate: bool,

    /// Structure extraction mode (UsingTree + N(...)/EOF DSL)
    #[arg(long, default_value_t = false)]
    structure: bool,
}

fn main() {
    let args = Args::parse();

    let cfg = utility::Config {
        src: args.src,
        dst: args.dst,
        include: args.include,
        exclude: args.exclude,
        max_per_file: args.max_per_file,
        skip_overrides: args.skip_overrides,
        skip_diagnostics: args.skip_diagnostics,
        use_new_emitter: args.new_emitter,
        prevalidate: if args.structure { false } else { args.prevalidate },
        structure_mode: args.structure,
    };

    if let Err(e) = generator::cli::dispatch_legacy(cfg) {
        eprintln!("Generator failed: {e}");
        std::process::exit(1);
    }
}
