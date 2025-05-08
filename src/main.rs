use anyhow::Result;
use env_logger::Env;

// Re-export the cli module from the library
use bsharp::cli;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    // Run the CLI application
    cli::run()
}

