use crate::commands::analyze::AnalyzeArgs;
use crate::commands::format::FormatArgs;
use crate::commands::parse::ParseArgs;
use crate::commands::tree::TreeArgs;
use crate::commands::rules::RulesArgs;
use crate::commands::{analyze, parse, tree};
use clap::{Parser, Subcommand};
use std::env;

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
    Parse(Box<ParseArgs>),

    /// Generate an AST visualization from a C# file (Mermaid/Graphviz)
    Tree(Box<TreeArgs>),

    /// Analyze a C# file, project or solution and print analysis results
    Analyze(Box<AnalyzeArgs>),

    /// Format .cs files under a path (file or directory) using the built-in formatter
    Format(Box<FormatArgs>),

    /// List analysis rulesets and rules
    Rules(Box<RulesArgs>),
}

pub fn run() -> anyhow::Result<()> {
    let os_args = env::args_os();
    let all_args =
        argfile::expand_args_from(os_args, argfile::parse_fromfile, argfile::PREFIX).unwrap();
    let cli = Cli::parse_from(all_args);

    match cli.command {
        Commands::Parse(args) => parse::execute(*args),

        Commands::Tree(args) => tree::execute(*args),

        Commands::Analyze(args) => analyze::execute(args.input.clone(), *args),

        Commands::Format(args) => crate::commands::format::execute(*args),

        Commands::Rules(args) => crate::commands::rules::execute(*args),
    }
}
