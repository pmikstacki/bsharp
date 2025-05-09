use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context, anyhow};

// Import from containing crate
use crate::parser;
use crate::parser::ast;

const SVG_HEADER: &str = "<?xml version=\"1.0\" standalone=\"no\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">";
const SVG_FOOTER: &str = "</svg>";

/// Execute the tree command: generate an SVG visualization of the AST
pub fn execute(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;
    
    // Parse the source code
    let parser = parser::Parser::new();
    let ast = parser.parse(&source_code)
        .map_err(|e| anyhow!("Parse error: {}", e))?;
    
    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension("svg");
        path
    });
    
    // Generate SVG content
    let svg_content = generate_svg_ast(&ast, &output_path)?;
    
    // Write the SVG to file
    fs::write(output_path.clone(), svg_content)
        .with_context(|| format!("Failed to write SVG file: {}", output_path.display()))?;
    
    println!("SVG AST visualization written to: {}", output_path.display());
    
    Ok(())
}

/// Generate an SVG representation of the AST
fn generate_svg_ast(_ast: &ast::CompilationUnit, output_path: &Path) -> Result<String> {
    // Create a new SVG document
    let mut document = svg::Document::new()
        .set("viewBox", (0, 0, 1200, 800))
        .set("width", "1200")
        .set("height", "800");
    
    // Add a title to the SVG
    let title = svg::node::element::Title::new().add(svg::node::Text::new("AST Visualization"));
    document = document.add(title);
    
    // Basic SVG styling for the document
    let style = svg::node::element::Style::new("
        .node { fill: #f8f9fa; stroke: #343a40; stroke-width: 2; }
        .node-text { font-family: 'Courier New', monospace; font-size: 14px; fill: #212529; }
        .edge { stroke: #6c757d; stroke-width: 1.5; }
    ");
    document = document.add(style);
    
    // Add a placeholder node for now (this would be expanded in a real implementation)
    let root_node = svg::node::element::Rectangle::new()
        .set("class", "node")
        .set("x", 500)
        .set("y", 50)
        .set("width", 200)
        .set("height", 50)
        .set("rx", 10)
        .set("ry", 10);
    document = document.add(root_node);
    
    // Add the root node text
    let root_text = svg::node::element::Text::new()
        .set("class", "node-text")
        .set("x", 600)
        .set("y", 75)
        .set("text-anchor", "middle")
        .add(svg::node::Text::new("CompilationUnit"));
    document = document.add(root_text);
    
    // TODO: Recursively visualize the AST tree structure here
    // This is a placeholder that would be expanded to traverse the 
    // AST and create a proper tree visualization
    
    // Save the SVG document to a string
    let svg_content = format!("{}{}{}", SVG_HEADER, document.to_string(), SVG_FOOTER);
    
    Ok(svg_content)
}
