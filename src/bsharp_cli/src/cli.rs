use std::env;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::commands::analyze::AnalyzeArgs;
use crate::commands::{analyze, parse, tree};
use crate::commands::format::FormatArgs;

#[derive(Parser)]
#[command(name = "bsharp")]
#[command(about = "BSharp CLI tool", version, author)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Parse a C# file and output the JSON representation of the parse tree
    Parse {
        /// The input C# file to parse
        #[arg(required = true)]
        input: PathBuf,

        /// The output JSON file (defaults to <input>.json)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Emit errors as JSON to stdout and exit with non-zero status (disables pretty errors)
        #[arg(long, default_value_t = false)]
        errors_json: bool,

        /// Disable ANSI colors in error output (pretty mode only)
        #[arg(long, default_value_t = false)]
        no_color: bool,

        /// Lenient mode: allow best-effort recovery (default: strict)
        #[arg(long, default_value_t = false)]
        lenient: bool,
    },

    /// Generate an AST visualization from a C# file (Mermaid/Graphviz)
    Tree {
        /// The input C# file to parse
        #[arg(required = true)]
        input: PathBuf,

        /// The output file (defaults to <input>.mmd for Mermaid or <input>.dot for Graphviz)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format: mermaid (default) or dot/graphviz
        #[arg(long, value_parser = ["mermaid", "dot", "graphviz"], default_value = "mermaid")]
        format: String,
    },

    /// Analyze a C# file, project or solution and print analysis results
    Analyze(Box<AnalyzeArgs>),

    /// Format .cs files under a path (file or directory) using the built-in formatter
    Format(Box<FormatArgs>)
}


pub fn run() -> anyhow::Result<()> {
    let os_args = env::args_os();
    let all_args = argfile::expand_args_from(
        os_args,
        argfile::parse_fromfile,
        argfile::PREFIX,
    ).unwrap();
    let cli = Cli::parse_from(all_args);

    match cli.command {
        Commands::Parse {
            input,
            output,
            errors_json,
            no_color,
            lenient,
        } => parse::execute(input, output, errors_json, no_color, lenient),

        Commands::Tree {
            input,
            output,
            format,
        } => tree::execute(input, output, format),

        Commands::Analyze(args) => analyze::execute(
            args.input.clone(),
            *args,
        ),

        Commands::Format(args) => crate::commands::format::execute(
           args
        ),
    }
}
