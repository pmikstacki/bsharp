use std::path::PathBuf;
use anyhow::{Result, anyhow};

// Import from containing crate
use crate::compiler;

/// Execute the compile command: compile a C# file
pub fn execute(input: PathBuf) -> Result<()> {
    // Create a compiler instance
    let mut compiler = compiler::Compiler::new();
    
    // Compile the file
    match compiler.compile_file(input.to_str().unwrap()) {
        Ok(_) => {
            println!("Compilation successful!");
            Ok(())
        },
        Err(e) => Err(anyhow!("Compilation failed: {}", e)),
    }
}
