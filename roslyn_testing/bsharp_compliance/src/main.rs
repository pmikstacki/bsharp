mod tests_writer;

use clap::Parser;
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
    };

    if let Err(e) = tests_writer::runner::run(cfg) {
        eprintln!("Generator failed: {e}");
        std::process::exit(1);
    }
}
