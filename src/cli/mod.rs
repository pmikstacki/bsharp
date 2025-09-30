pub mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use self::commands::{compile, parse, tree, analyze};

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
    },

    /// Generate an SVG visualization of the AST from a C# file
    Tree {
        /// The input C# file to parse
        #[arg(required = true)]
        input: PathBuf,

        /// The output SVG file (defaults to <input>.svg)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Compile a C# file
    Compile {
        /// The input C# file to compile
        #[arg(required = true)]
        input: PathBuf,
    },

    /// Analyze a C# file, project or solution and print analysis results
    Analyze {
        /// The input C# file to analyze
        #[arg(required = true)]
        input: PathBuf,

        /// Optional symbol name to search for; if omitted, prints all top-level declaration spans
        #[arg(short, long)]
        symbol: Option<String>,

        /// Optional analysis config file (JSON/TOML)
        #[arg(long)]
        config: Option<PathBuf>,

        /// Optional output file path for the analysis JSON report
        #[arg(long, value_name = "FILE")]
        out: Option<PathBuf>,

        /// Follow ProjectReference dependencies (default: true)
        #[arg(long, default_value_t = true)]
        follow_refs: bool,

        /// Include only files matching these globs (workspace mode). Multiple allowed.
        #[arg(long, value_name = "GLOB", num_args = 0..)]
        include: Vec<String>,

        /// Exclude files matching these globs (workspace mode). Multiple allowed.
        #[arg(long, value_name = "GLOB", num_args = 0..)]
        exclude: Vec<String>,

        /// Output format: json (compact) or pretty-json (default)
        #[arg(long, value_parser = ["json", "pretty-json"], default_value = "pretty-json")]
        format: String,

        /// Enable specific rulesets by id (multiple allowed)
        #[arg(long, value_name = "ID", num_args = 0..)]
        enable_ruleset: Vec<String>,

        /// Disable specific rulesets by id (multiple allowed)
        #[arg(long, value_name = "ID", num_args = 0..)]
        disable_ruleset: Vec<String>,

        /// Enable specific passes by id (multiple allowed)
        #[arg(long, value_name = "ID", num_args = 0..)]
        enable_pass: Vec<String>,

        /// Disable specific passes by id (multiple allowed)
        #[arg(long, value_name = "ID", num_args = 0..)]
        disable_pass: Vec<String>,

        /// Override severities: CODE=level (level: error|warning|info|hint); multiple allowed
        #[arg(long, value_name = "PAIR", num_args = 0..)]
        severity: Vec<String>,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { input, output, errors_json, no_color } => parse::execute(input, output, errors_json, no_color),

        Commands::Tree { input, output } => tree::execute(input, output),

        Commands::Compile { input } => compile::execute(input),

        Commands::Analyze { input, symbol, config, out, follow_refs, include, exclude, format, enable_ruleset, disable_ruleset, enable_pass, disable_pass, severity } =>
            analyze::execute(input, symbol, config, out, follow_refs, include, exclude, format, enable_ruleset, disable_ruleset, enable_pass, disable_pass, severity),
    }
}
