pub mod commands;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use self::commands::{analyze, parse, tree};
use env_logger::Env;

#[derive(Parser)]
#[command(name = "bsharp")]
#[command(about = "BSharp CLI tool", version, author)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl From<&AnalyzeArgs> for analyze::AnalyzeOptions {
    fn from(args: &AnalyzeArgs) -> Self {
        analyze::AnalyzeOptions {
            symbol: args.symbol.clone(),
            config: args.config.clone(),
            out: args.out.clone(),
            follow_refs: args.follow_refs,
            include: args.include.clone(),
            exclude: args.exclude.clone(),
            format: args.format.clone(),
            enable_ruleset: args.enable_ruleset.clone(),
            disable_ruleset: args.disable_ruleset.clone(),
            enable_pass: args.enable_pass.clone(),
            disable_pass: args.disable_pass.clone(),
            severity: args.severity.clone(),
        }
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    run()
}

#[derive(Args, Debug, Clone)]
pub struct AnalyzeArgs {
    /// The input C# file to analyze
    #[arg(required = true)]
    pub input: PathBuf,

    /// Optional symbol name to search for; if omitted, prints all top-level declaration spans
    #[arg(short, long)]
    pub symbol: Option<String>,

    /// Optional analysis config file (JSON/TOML)
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Optional output file path for the analysis JSON report
    #[arg(long, value_name = "FILE")]
    pub out: Option<PathBuf>,

    /// Follow ProjectReference dependencies (default: true)
    #[arg(long, default_value_t = true)]
    pub follow_refs: bool,

    /// Include only files matching these globs (workspace mode). Multiple allowed.
    #[arg(long, value_name = "GLOB", num_args = 0..)]
    pub include: Vec<String>,

    /// Exclude files matching these globs (workspace mode). Multiple allowed.
    #[arg(long, value_name = "GLOB", num_args = 0..)]
    pub exclude: Vec<String>,

    /// Output format: json (compact) or pretty-json (default)
    #[arg(long, value_parser = ["json", "pretty-json"], default_value = "pretty-json")]
    pub format: String,

    /// Enable specific rulesets by id (multiple allowed)
    #[arg(long, value_name = "ID", num_args = 0..)]
    pub enable_ruleset: Vec<String>,

    /// Disable specific rulesets by id (multiple allowed)
    #[arg(long, value_name = "ID", num_args = 0..)]
    pub disable_ruleset: Vec<String>,

    /// Enable specific passes by id (multiple allowed)
    #[arg(long, value_name = "ID", num_args = 0..)]
    pub enable_pass: Vec<String>,

    /// Disable specific passes by id (multiple allowed)
    #[arg(long, value_name = "ID", num_args = 0..)]
    pub disable_pass: Vec<String>,

    /// Override severities: CODE=level (level: error|warning|info|hint); multiple allowed
    #[arg(long, value_name = "PAIR", num_args = 0..)]
    pub severity: Vec<String>,
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
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

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
            analyze::AnalyzeOptions::from(args.as_ref()),
        ),
    }
}
