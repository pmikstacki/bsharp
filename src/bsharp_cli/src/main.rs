mod cli;
pub mod commands;
pub mod errors;

use anyhow::Result;

use env_logger::Env;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    cli::run()
}
