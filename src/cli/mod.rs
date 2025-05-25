pub mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use self::commands::{compile, parse, tree};

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
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Parse { input, output } => {
            parse::execute(input, output)
        },
        
        Commands::Tree { input, output } => {
            tree::execute(input, output)
        },
        
        Commands::Compile { input } => {
            compile::execute(input)
        },
    }
}
