pub mod commands;
mod cli;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use self::commands::{analyze, parse, tree, format as fmt_cmd};
use env_logger::Env;
use crate::cli::{Cli, Commands};
use crate::commands::analyze::AnalyzeArgs;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    cli::run()
}

