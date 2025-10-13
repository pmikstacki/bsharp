use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

// Import the syntax from the containing crate
use bsharp_analysis::diagnostics::parse as diag_parse;
use bsharp_parser::bsharp::{parse_csharp_source, parse_csharp_source_strict};
use bsharp_parser::expressions::statements::UsingDirective;
use bsharp_parser::helpers::brace_tracker;
use bsharp_parser::parse_mode;
use bsharp_parser::syntax::ast;
use nom::Finish;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
use std::env;

/// Execute the parse command: parse C# file and output JSON
/// On parse failure, pretty-print errors by default and exit non-zero.
/// If `errors_json` is true, emit a JSON error object to stdout instead.
/// If `no_color` is true (or NO_COLOR is set), ANSI colors are disabled in pretty output.
pub fn execute(
    input: PathBuf,
    _output: Option<PathBuf>,
    errors_json: bool,
    no_color: bool,
    lenient: bool,
) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    // Select strict or lenient parser and set strict flag for deep parsers
    let parser = if lenient {
        parse_csharp_source
    } else {
        parse_csharp_source_strict
    };
    let prev_strict = parse_mode::is_strict();
    parse_mode::set_strict(!lenient);

    // Parse the source code (low-level) to preserve structured errors
    let parse_result = parser(&source_code).finish();
    // Restore previous strict mode to avoid leaking state
    parse_mode::set_strict(prev_strict);
    let (remaining, ast) = match parse_result {
        Ok(ok) => ok,
        Err(e) => {
            // Pretty body for stderr or message field
            let pretty = diag_parse::render_pretty_parse_error(&source_code, &e);
            if errors_json {
                let (line, column, line_text) = diag_parse::summarize_location(&source_code, &e);
                let (expected, found) = diag_parse::summarize_expected_found(&e);
                let payload = serde_json::json!({
                    "error": {
                        "kind": "parse_error",
                        "file": input.display().to_string(),
                        "line": line,
                        "column": column,
                        "expected": expected,
                        "found": found,
                        "line_text": line_text,
                        "message": pretty
                    }
                });
                println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
            } else {
                print_pretty_error(&input, &pretty, no_color);
            }
            std::process::exit(1);
        }
    };

    // Treat significant trailing input as a pretty error with location and caret
    if !lenient && !remaining.trim().is_empty() {
        if let Some(status) = brace_tracker::take_status() {
            if let Some(offset) = status.unmatched_open {
                let tree = ErrorTree::Base {
                    location: &source_code[offset..],
                    kind: BaseErrorKind::Expected(Expectation::Char('}')),
                };
                let pretty = diag_parse::render_pretty_parse_error(&source_code, &tree);
                if errors_json {
                    let (line, column, line_text) =
                        diag_parse::summarize_location(&source_code, &tree);
                    let (expected, found) = diag_parse::summarize_expected_found(&tree);
                    let payload = serde_json::json!({
                        "error": {
                            "kind": "parse_error",
                            "file": input.display().to_string(),
                            "line": line,
                            "column": column,
                            "expected": expected,
                            "found": found,
                            "line_text": line_text,
                            "message": pretty
                        }
                    });
                    println!(
                        "{}",
                        serde_json::to_string(&payload).unwrap_or_else(|_| {
                            "{\"error\":{\"message\":\"parse error\"}}".to_string()
                        })
                    );
                } else {
                    print_pretty_error(&input, &pretty, no_color);
                }
            } else {
                // Build a synthetic EOF-expected error at the remaining location
                let tree = ErrorTree::Base {
                    location: remaining,
                    kind: BaseErrorKind::Expected(Expectation::Eof),
                };
                let pretty = diag_parse::render_pretty_parse_error(&source_code, &tree);
                if errors_json {
                    let (line, column, line_text) =
                        diag_parse::summarize_location(&source_code, &tree);
                    let (expected, found) = diag_parse::summarize_expected_found(&tree);
                    let payload = serde_json::json!({
                        "error": {
                            "kind": "parse_error",
                            "file": input.display().to_string(),
                            "line": line,
                            "column": column,
                            "expected": expected,
                            "found": found,
                            "line_text": line_text,
                            "message": pretty
                        }
                    });
                    println!(
                        "{}",
                        serde_json::to_string(&payload).unwrap_or_else(|_| {
                            "{\"error\":{\"message\":\"parse error\"}}".to_string()
                        })
                    );
                } else {
                    print_pretty_error(&input, &pretty, no_color);
                }
            }
        } else {
            // Build a synthetic EOF-expected error at the remaining location
            let tree = ErrorTree::Base {
                location: remaining,
                kind: BaseErrorKind::Expected(Expectation::Eof),
            };
            let pretty = diag_parse::render_pretty_parse_error(&source_code, &tree);
            if errors_json {
                let (line, column, line_text) = diag_parse::summarize_location(&source_code, &tree);
                let (expected, found) = diag_parse::summarize_expected_found(&tree);
                let payload = serde_json::json!({
                    "error": {
                        "kind": "parse_error",
                        "file": input.display().to_string(),
                        "line": line,
                        "column": column,
                        "expected": expected,
                        "found": found,
                        "line_text": line_text,
                        "message": pretty
                    }
                });
                println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
            } else {
                print_pretty_error(&input, &pretty, no_color);
            }
        }
        std::process::exit(1);
    }

    // Produce a formatted textual tree representation and print to stdout
    let tree = format_text_tree(&ast);
    println!("{}", tree);

    Ok(())
}

fn print_pretty_error(input: &Path, pretty_body: &str, no_color_flag: bool) {
    let no_color_env = env::var("NO_COLOR").is_ok();
    let colored = !(no_color_flag || no_color_env);

    let (err_hdr, arrow) = if colored {
        ("\x1b[1;31merror\x1b[0m:", "\x1b[31m^\x1b[0m")
    } else {
        ("error:", "^")
    };
    let file = if colored {
        format!("\x1b[36m{}\x1b[0m", input.display())
    } else {
        format!("{}", input.display())
    };

    // pretty_body already includes line/col and caret; optionally we can recolor caret by replacing '^'
    // For simplicity, prepend header and file path; leave body as-is.
    eprintln!(
        "{} failed to parse file\n  --> {}\n{}",
        err_hdr,
        file,
        pretty_body.replace('^', arrow)
    );
}

fn format_text_tree(ast: &ast::CompilationUnit) -> String {
    let mut out = String::new();
    use std::fmt::Write as _;

    writeln!(&mut out, "CompilationUnit").ok();
    writeln!(&mut out, "  Usings ({}):", ast.using_directives.len()).ok();
    for using in &ast.using_directives {
        writeln!(&mut out, "    - {}", format_using_directive(using)).ok();
    }

    writeln!(&mut out, "  Declarations ({}):", ast.declarations.len()).ok();
    for decl in &ast.declarations {
        let label = match decl {
            ast::TopLevelDeclaration::Namespace(ns) => format!("Namespace: {}", ns.name.name),
            ast::TopLevelDeclaration::FileScopedNamespace(fns) => {
                format!("File-Scoped Namespace: {}", fns.name.name)
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
        writeln!(&mut out, "    - {}", label).ok();
    }

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
