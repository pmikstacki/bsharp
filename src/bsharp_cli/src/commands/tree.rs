use anyhow::{anyhow, Context, Result};
use nom::Finish;
use std::fs;
use std::path::PathBuf;

// Import from containing crate
use bsharp_parser::bsharp::parse_csharp_source;
use bsharp_parser::expressions::statements::UsingDirective;
use bsharp_parser::parse_mode;
use bsharp_parser::syntax::ast;
use std::sync::{Mutex, OnceLock};

/// Execute the tree command: generate a Mermaid (default) or Graphviz (DOT) visualization of the AST
pub fn execute(input: PathBuf, output: Option<PathBuf>, format: String) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    // Parse the source code using lenient parser to maximize visualization coverage.
    // Guard global parse_mode with a process-wide mutex to avoid races in parallel tests.
    static PARSE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let lock = PARSE_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().unwrap();
    let prev_strict = parse_mode::is_strict();
    parse_mode::set_strict(false);
    let parse_result = parse_csharp_source(&source_code).finish();
    parse_mode::set_strict(prev_strict);
    let (_remaining, ast) = parse_result.map_err(|e| anyhow!("Parse error: {:?}", e))?;

    // Determine output path based on requested format
    let fmt = format.to_lowercase();
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        match fmt.as_str() {
            "dot" | "graphviz" => path.set_extension("dot"),
            _ => path.set_extension("mmd"), // default to Mermaid extension
        };
        path
    });

    // Generate content for selected format
    let content = match fmt.as_str() {
        "dot" | "graphviz" => generate_dot_ast(&ast),
        _ => generate_mermaid_ast(&ast),
    };

    // Write the content to file (explicitly flush and sync to avoid 0-byte files)
    use std::io::Write;
    let mut file = fs::File::create(output_path.clone())
        .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;
    file.write_all(content.as_bytes())
        .with_context(|| format!("Failed to write output content: {}", output_path.display()))?;
    file.sync_all()
        .with_context(|| format!("Failed to sync output file: {}", output_path.display()))?;

    println!(
        "AST visualization (format: {}) written to: {}",
        fmt,
        output_path.display()
    );

    Ok(())
}

/// Generate a Mermaid representation of the AST (simple, top-level only for now)
fn generate_mermaid_ast(ast: &ast::CompilationUnit) -> String {
    let mut lines = Vec::new();
    lines.push("graph TD".to_string());

    let root_id = "n0".to_string();
    let root_label = format!(
        "CompilationUnit\\nUsings: {}\\nDecls: {}",
        ast.using_directives.len(),
        ast.declarations.len()
    );
    lines.push(format!("{root_id}[\"{root_label}\"]"));

    // Usings
    for (i, using) in ast.using_directives.iter().enumerate() {
        let id = format!("u{}", i);
        let label = format!("Using {}", format_using_directive(using));
        lines.push(format!("{id}[\"{label}\"]"));
        lines.push(format!("{root_id} --> {id}"));
    }

    // Top-level declarations
    for (i, member) in ast.declarations.iter().enumerate() {
        let id = format!("d{}", i);
        let label = match member {
            ast::TopLevelDeclaration::Namespace(ns) => format!("Namespace: {}", ns.name.name),
            ast::TopLevelDeclaration::FileScopedNamespace(file_scoped_ns) => {
                format!("File-Scoped Namespace: {}", file_scoped_ns.name.name)
            }
            ast::TopLevelDeclaration::Class(cl) => format!("Class: {}", cl.name.name),
            ast::TopLevelDeclaration::Interface(iface) => format!("Interface: {}", iface.name.name),
            ast::TopLevelDeclaration::Struct(st) => format!("Struct: {}", st.name.name),
            ast::TopLevelDeclaration::Enum(enum_decl) => format!("Enum: {}", enum_decl.name.name),
            ast::TopLevelDeclaration::Record(rec) => format!("Record: {}", rec.name.name),
            ast::TopLevelDeclaration::Delegate(del) => format!("Delegate: {}", del.name.name),
            ast::TopLevelDeclaration::GlobalAttribute(global_attr) => format!(
                "Global Attribute: {} -> {}",
                global_attr.target.name, global_attr.attribute.name.name
            ),
        };
        lines.push(format!("{id}[\"{label}\"]"));
        lines.push(format!("{root_id} --> {id}"));
    }

    lines.join("\n")
}

/// Generate a Graphviz DOT representation of the AST (simple, top-level only for now)
fn generate_dot_ast(ast: &ast::CompilationUnit) -> String {
    let mut out = String::new();
    out.push_str("digraph AST {\n");
    out.push_str("  node [shape=box, fontname=\"Courier New\"];\n");

    let root_id = "n0";
    let root_label = format!(
        "CompilationUnit\\nUsings: {}\\nDecls: {}",
        ast.using_directives.len(),
        ast.declarations.len()
    );
    out.push_str(&format!("  {root_id} [label=\"{root_label}\"];\n"));

    for (i, using) in ast.using_directives.iter().enumerate() {
        let id = format!("u{}", i);
        let label = format!("Using {}", format_using_directive(using));
        out.push_str(&format!("  {id} [label=\"{label}\"];\n"));
        out.push_str(&format!("  {root_id} -> {id};\n"));
    }

    for (i, member) in ast.declarations.iter().enumerate() {
        let id = format!("d{}", i);
        let label = match member {
            ast::TopLevelDeclaration::Namespace(ns) => format!("Namespace: {}", ns.name.name),
            ast::TopLevelDeclaration::FileScopedNamespace(file_scoped_ns) => {
                format!("File-Scoped Namespace: {}", file_scoped_ns.name.name)
            }
            ast::TopLevelDeclaration::Class(cl) => format!("Class: {}", cl.name.name),
            ast::TopLevelDeclaration::Interface(iface) => format!("Interface: {}", iface.name.name),
            ast::TopLevelDeclaration::Struct(st) => format!("Struct: {}", st.name.name),
            ast::TopLevelDeclaration::Enum(enum_decl) => format!("Enum: {}", enum_decl.name.name),
            ast::TopLevelDeclaration::Record(rec) => format!("Record: {}", rec.name.name),
            ast::TopLevelDeclaration::Delegate(del) => format!("Delegate: {}", del.name.name),
            ast::TopLevelDeclaration::GlobalAttribute(global_attr) => format!(
                "Global Attribute: {} -> {}",
                global_attr.target.name, global_attr.attribute.name.name
            ),
        };
        out.push_str(&format!("  {id} [label=\"{label}\"];\n"));
        out.push_str(&format!("  {root_id} -> {id};\n"));
    }

    out.push_str("}\n");
    out
}

fn format_using_directive(using: &UsingDirective) -> String {
    match using {
        UsingDirective::Namespace { namespace } => format!("using {};", namespace.name),
        UsingDirective::Alias {
            alias,
            namespace_or_type,
        } => format!("using {} = {};", alias.name, namespace_or_type.name),
        UsingDirective::Static { type_name } => format!("using static {};", type_name.name),
    }
}
