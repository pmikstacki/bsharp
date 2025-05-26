use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

// Import from containing crate
use crate::parser;
use crate::parser::ast;
use crate::parser::nodes::declarations::UsingDirective;

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
fn generate_svg_ast(ast: &ast::CompilationUnit, _output_path: &Path) -> Result<String> {
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
    
    // Add a root node representing CompilationUnit
    let cu_node = svg::node::element::Rectangle::new()
        .set("class", "node")
        .set("x", 450)
        .set("y", 50)
        .set("width", 300) // Increased width for more text
        .set("height", 50)
        .set("rx", 10)
        .set("ry", 10);
    document = document.add(cu_node);
    
    // Add text for CompilationUnit details
    let cu_text_content = format!("CompilationUnit (Using Directives: {}, Declarations: {})", ast.using_directives.len(), ast.declarations.len());
    let cu_text = svg::node::element::Text::new()
        .set("class", "node-text")
        .set("x", 600) // Centered within the new width
        .set("y", 75)
        .set("text-anchor", "middle")
        .add(svg::node::Text::new(cu_text_content));
    document = document.add(cu_text);
    
    // Add usings as children
    for (i, using) in ast.using_directives.iter().enumerate() {
        let using_label = format!("Using[{}]: {}", i, format_using_directive(using));
        let using_node = svg::node::element::Rectangle::new()
            .set("class", "node")
            .set("x", 100.0)
            .set("y", 120.0 + i as f64 * 40.0)
            .set("width", 250.0)
            .set("height", 30.0)
            .set("rx", 5.0)
            .set("ry", 5.0);
        document = document.add(using_node);

        let using_text = svg::node::element::Text::new()
            .set("class", "node-text")
            .set("x", 225.0) // Centered
            .set("y", 120.0 + i as f64 * 40.0)
            .set("text-anchor", "middle")
            .add(svg::node::Text::new(using_label));
        document = document.add(using_text);
    }
    
    // Add top-level members as children
    for (i, member) in ast.declarations.iter().enumerate() {
        // Create unique IDs for members to avoid graphviz errors
        // let member_id = format!("cu_member_{}", i);
        // For now, let's just use the member name as ID and hope it's unique enough
        // for this visualization. If not, we might need to generate unique IDs.
        let _member_id = format!("cu_member_{}", i); // Prefixed with underscore

        let member_label = match member {
            ast::TopLevelDeclaration::Namespace(ns) => format!("Namespace: {}", ns.name.name),
            ast::TopLevelDeclaration::FileScopedNamespace(file_scoped_ns) => format!("File-Scoped Namespace: {}", file_scoped_ns.name.name),
            ast::TopLevelDeclaration::Class(cl) => format!("Class: {}", cl.name.name),
            ast::TopLevelDeclaration::Interface(iface) => format!("Interface: {}", iface.name.name),
            ast::TopLevelDeclaration::Struct(st) => format!("Struct: {}", st.name.name),
            ast::TopLevelDeclaration::Enum(enum_decl) => format!("Enum: {}", enum_decl.name.name),
            ast::TopLevelDeclaration::Record(rec) => format!("Record: {}", rec.name.name),
            ast::TopLevelDeclaration::Delegate(del) => format!("Delegate: {}", del.name.name),
            ast::TopLevelDeclaration::GlobalAttribute(global_attr) => format!("Global Attribute: {} -> {}", global_attr.target.name, global_attr.attribute.name.name),
        };

        let member_node = svg::node::element::Rectangle::new()
            .set("class", "node")
            .set("x", 100.0)
            .set("y", 120.0 + ast.using_directives.len() as f64 * 40.0 + i as f64 * 40.0)
            .set("width", 250.0)
            .set("height", 30.0)
            .set("rx", 5.0)
            .set("ry", 5.0);
        document = document.add(member_node);

        let member_text = svg::node::element::Text::new()
            .set("class", "node-text")
            .set("x", 225.0) // Centered
            .set("y", 120.0 + ast.using_directives.len() as f64 * 40.0 + i as f64 * 40.0)
            .set("text-anchor", "middle")
            .add(svg::node::Text::new(member_label));
        document = document.add(member_text);
    }
    
    // Save the SVG document to a string
    let svg_content = format!("{}{}{}", SVG_HEADER, document.to_string(), SVG_FOOTER);
    
    Ok(svg_content)
}

fn format_using_directive(using: &UsingDirective) -> String {
    match using {
        UsingDirective::Namespace { namespace } => format!("using {};", namespace.name),
        UsingDirective::Alias { alias, namespace_or_type } => format!("using {} = {};", alias.name, namespace_or_type.name),
        UsingDirective::Static { type_name } => format!("using static {};", type_name.name),
    }
}
