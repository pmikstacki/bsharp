use std::path::PathBuf;

/// Test helper equivalent of CLI parse command. Reads C# file, parses, writes JSON AST.
pub fn parse_execute(
    input: PathBuf,
    output: Option<PathBuf>,
    _errors_json: bool,
    _no_color: bool,
    lenient: bool,
) -> anyhow::Result<()> {
    use anyhow::{anyhow, Context};
    use nom::Finish;
    use parser::bsharp::{parse_csharp_source, parse_csharp_source_strict};
    use parser::parse_mode;
    use std::fs;

    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    let parser = if lenient {
        parse_csharp_source
    } else {
        parse_csharp_source_strict
    };
    let prev_strict = parse_mode::is_strict();
    parse_mode::set_strict(!lenient);
    let parsed = parser(&source_code).finish();
    parse_mode::set_strict(prev_strict);
    let (_remaining, ast) = parsed.map_err(|e| anyhow!("Parse error: {:?}", e))?;

    let output_path = output.unwrap_or_else(|| {
        let mut p = input.clone();
        p.set_extension("json");
        p
    });
    let json = serde_json::to_string_pretty(&ast).context("Failed to serialize AST to JSON")?;
    fs::write(&output_path, json)
        .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;
    Ok(())
}

/// Test helper equivalent of CLI tree command. Generates Mermaid or DOT AST graph.
pub fn tree_execute(input: PathBuf, output: Option<PathBuf>, format: String) -> anyhow::Result<()> {
    use anyhow::{anyhow, Context};
    use nom::Finish;
    use parser::bsharp::parse_csharp_source;
    use parser::parse_mode;
    use parser::syntax::{ast, declarations::UsingDirective};
    use std::fs;

    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    let prev = parse_mode::is_strict();
    parse_mode::set_strict(false);
    let parsed = parse_csharp_source(&source_code).finish();
    parse_mode::set_strict(prev);
    let (_remaining, ast) = parsed.map_err(|e| anyhow!("Parse error: {:?}", e))?;

    let fmt = format.to_lowercase();
    let output_path = output.unwrap_or_else(|| {
        let mut p = input.clone();
        match fmt.as_str() {
            "dot" | "graphviz" => p.set_extension("dot"),
            _ => p.set_extension("mmd"),
        };
        p
    });

    // Generators (minimal copy from CLI)
    fn format_using_directive(using: &UsingDirective) -> String {
        match using {
            UsingDirective::Namespace { namespace } => format!("using {};", namespace.name),
            UsingDirective::Alias {
                alias,
                namespace_or_type,
            } => {
                format!("using {} = {};", alias.name, namespace_or_type.name)
            }
            UsingDirective::Static { type_name } => format!("using static {};", type_name.name),
        }
    }
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
        for (i, using) in ast.using_directives.iter().enumerate() {
            let id = format!("u{}", i);
            let label = format!("Using {}", format_using_directive(using));
            lines.push(format!("{id}[\"{label}\"]"));
            lines.push(format!("{root_id} --> {id}"));
        }
        for (i, member) in ast.declarations.iter().enumerate() {
            let id = format!("d{}", i);
            let label = match member {
                ast::TopLevelDeclaration::Namespace(ns) => format!("Namespace: {}", ns.name.name),
                ast::TopLevelDeclaration::FileScopedNamespace(fs) => {
                    format!("File-Scoped Namespace: {}", fs.name.name)
                }
                ast::TopLevelDeclaration::Class(cl) => format!("Class: {}", cl.name.name),
                ast::TopLevelDeclaration::Interface(iface) => {
                    format!("Interface: {}", iface.name.name)
                }
                ast::TopLevelDeclaration::Struct(st) => format!("Struct: {}", st.name.name),
                ast::TopLevelDeclaration::Enum(en) => format!("Enum: {}", en.name.name),
                ast::TopLevelDeclaration::Record(rec) => format!("Record: {}", rec.name.name),
                ast::TopLevelDeclaration::Delegate(del) => format!("Delegate: {}", del.name.name),
                ast::TopLevelDeclaration::GlobalAttribute(ga) => format!(
                    "Global Attribute: {} -> {}",
                    ga.target.name, ga.attribute.name.name
                ),
            };
            lines.push(format!("{id}[\"{label}\"]"));
            lines.push(format!("{root_id} --> {id}"));
        }
        lines.join("\n")
    }
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
                ast::TopLevelDeclaration::FileScopedNamespace(fs) => {
                    format!("File-Scoped Namespace: {}", fs.name.name)
                }
                ast::TopLevelDeclaration::Class(cl) => format!("Class: {}", cl.name.name),
                ast::TopLevelDeclaration::Interface(iface) => {
                    format!("Interface: {}", iface.name.name)
                }
                ast::TopLevelDeclaration::Struct(st) => format!("Struct: {}", st.name.name),
                ast::TopLevelDeclaration::Enum(en) => format!("Enum: {}", en.name.name),
                ast::TopLevelDeclaration::Record(rec) => format!("Record: {}", rec.name.name),
                ast::TopLevelDeclaration::Delegate(del) => format!("Delegate: {}", del.name.name),
                ast::TopLevelDeclaration::GlobalAttribute(ga) => format!(
                    "Global Attribute: {} -> {}",
                    ga.target.name, ga.attribute.name.name
                ),
            };
            out.push_str(&format!("  {id} [label=\"{label}\"];\n"));
            out.push_str(&format!("  {root_id} -> {id};\n"));
        }
        out.push_str("}\n");
        out
    }

    let content = match fmt.as_str() {
        "dot" | "graphviz" => generate_dot_ast(&ast),
        _ => generate_mermaid_ast(&ast),
    };
    fs::write(&output_path, content)
        .with_context(|| format!("Failed to write output: {}", output_path.display()))?;
    Ok(())
}
