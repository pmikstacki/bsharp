use crate::codegen::CodeGenerator;
use crate::syntax::Parser;
use anyhow::{anyhow, Result};
use log::{debug, error, info};
use std::{fs, path::Path};

pub struct Compiler {
    parser: Parser,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    pub fn compile_file(&mut self, path: &str) -> Result<()> {
        let source = fs::read_to_string(path)?;
        
        info!("Parsing source file: {}", path);
        let ast = self.parser.parse(&source).map_err(|e: String| {
            error!("Parsing failed: {}", e);
            anyhow!("Parser error: {}", e)
        })?;
        debug!("AST generated successfully: {:?}", ast);

        let codegen = CodeGenerator::new();
        info!("Generating code");
        let bytecode = codegen.compile(&ast).map_err(|e: String| {
            error!("Code generation failed: {}", e);
            anyhow!("Codegen error: {}", e)
        })?;
        info!("Bytecode generated successfully ({} bytes).", bytecode.len());

        let output_path = Path::new(path).with_extension("o");
        info!("Writing object file to: {:?}", output_path);
        fs::write(&output_path, &bytecode)?;

        info!("Successfully compiled to: {}", output_path.display());
        Ok(())
    }
}
