// C# source file syntax - following Roslyn naming conventions
// Nodes in Roslyn use the "Syntax" suffix (e.g., NamespaceDeclarationSyntax)

use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;

// parser_helpers imported selectively in sub-parser; this module only needs ws
use crate::parser::expressions::declarations::file_scoped_namespace_parser::parse_file_scoped_namespace_declaration;
use crate::parser::expressions::declarations::global_attribute_parser::parse_global_attributes;
use crate::parser::expressions::declarations::namespace_declaration_parser::parse_namespace_declaration;
use crate::parser::expressions::declarations::type_declaration_parser::parse_type_declaration;
use crate::parser::expressions::declarations::using_directive_parser::parse_using_directive;
use crate::parser::expressions::statements::top_level_statement_parser::parse_top_level_statements;
use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::comment_parser::ws;
use crate::syntax::parser_helpers::{bpeek, bws, keyword};
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::TypeDeclaration;
use log::trace;

/// Parse a C# source file following Roslyn's model where a source file contains:
/// - global attributes (assembly/module attributes)
/// - using directives (imports)
/// - optional file-scoped namespace (C# 10+)
/// - namespace or top-level type declarations
/// - optional top-level statements (C# 9+)
pub fn parse_csharp_source(input: &str) -> BResult<&str, CompilationUnit> {
    trace!(
        "Starting source file parsing with input length: {}",
        input.len()
    );

    // Skip any leading whitespace/comments and remove any BOM or other text markers
    let (remaining, _) = ws(input)?;

    if log::log_enabled!(log::Level::Trace) {
        // Trace the input start (debug-only)
        let preview = remaining.chars().take(80).collect::<String>();
        trace!("Parsing file starting with: {}", preview);
        let input_lines: Vec<&str> = input.lines().collect();
        trace!("Input has {} lines", input_lines.len());
        for (i, line) in input_lines.iter().take(5).enumerate() {
            trace!("Line {}: {}", i, line);
        }
    }

    // Parse global attributes first (assembly and module attributes)
    let (remaining, global_attributes) = parse_global_attributes(remaining)?;
    if !global_attributes.is_empty() {
        trace!(
            "Successfully parsed {} global attributes",
            global_attributes.len()
        );
    }

    // Parse using directives next (including optional 'global using' which we normalize to UsingDirective)
    let mut remaining = remaining;
    let mut usings = Vec::new();
    loop {
        // Skip whitespace/comments between directives
        let (r, _) = ws(remaining)?;
        remaining = r;
        // Look for 'global using' or 'using'
        if bpeek(keyword("global"))(remaining).is_ok() {
            // consume 'global'
            let (r, _) = bws(keyword("global"))(remaining)?;
            // next must be 'using'
            if bpeek(keyword("using"))(r).is_ok() {
                let (r2, _) = bws(keyword("using"))(r)?;
                // parse the rest of using directive by reusing parser on the remainder that starts after 'using'
                // We need a helper that expects we've already consumed 'using', but to keep it simple,
                // rebuild input by prefixing 'using ' back is cumbersome. Instead, call parse_using_directive starting at 'using'.
                // So we back up to a string that starts with 'using' by ignoring the consumed token above.
                // As a simpler approach, parse_using_directive from remaining after 'global' by expecting 'using' again.
                // We'll emulate by constructing a small closure.
                let using_input = format!("using{}", r2);
                // Not ideal to allocate, fallback: call parse_using_directive on original input (r) which still begins with 'using'.
                // We already consumed 'using' into r2, but we can instead not consume 'using' and just call the parser on r.
                let (r_after, using_dir) = parse_using_directive(r)?;
                usings.push(using_dir);
                remaining = r_after;
                continue;
            } else {
                // Not a 'using' after global; stop scanning usings
                break;
            }
        } else if bpeek(keyword("using"))(remaining).is_ok() {
            let (r_after, using_dir) = bws(parse_using_directive)(remaining)?;
            usings.push(using_dir);
            remaining = r_after;
            continue;
        } else {
            break;
        }
    }
    if log::log_enabled!(log::Level::Trace) && !usings.is_empty() {
        trace!(
            "Successfully parsed {} using directives",
            usings.len()
        );
    }

    // Try to parse file-scoped namespace (C# 10+)
    let mut file_scoped_namespace = None;

    // Prefer lookahead-based detection rather than string peeks
    if bpeek(keyword("namespace"))(remaining).is_ok() {
        if let Ok((rest, namespace)) = parse_file_scoped_namespace_declaration(remaining) {
            if log::log_enabled!(log::Level::Trace) {
                trace!(
                    "Successfully parsed file-scoped namespace: {}",
                    namespace.name.name
                );
            }
            file_scoped_namespace = Some(namespace);
            remaining = rest;
        }
    }

    // Now parse top-level members (namespaces, classes)
    let mut members = Vec::new();
    let mut top_level_statements = Vec::new();

    // Skip any whitespace between using directives and top-level members
    let (mut remaining, _) = ws(remaining)?;

    // If there's still input, try to parse top-level members or statements
    while !remaining.trim().is_empty() {
        // First try to parse top-level members (classes, namespaces, etc.)
        match parse_top_level_member(remaining) {
            Ok((rest, member)) => {
                trace!("Successfully parsed top-level member");
                members.push(member);
                remaining = rest;

                // Skip any whitespace between members
                let (after_ws, _) = ws(remaining)?;
                remaining = after_ws;
            }
            Err(_) => {
                // If we can't parse a member, try to parse top-level statements (C# 9+)
                match parse_top_level_statements(remaining) {
                    Ok((rest, statements)) => {
                        trace!(
                            "Successfully parsed {} top-level statements",
                            statements.len()
                        );
                        top_level_statements.extend(statements);
                        remaining = rest;
                        break; // Top-level statements consume the rest of the file
                    }
                    Err(e) => {
                        trace!("Failed to parse top-level member or statements: {:?}", e);
                        break;
                    }
                }
            }
        }
    }

    // Create the source file
    let compilation_unit = CompilationUnit {
        using_directives: usings,
        declarations: members,
        file_scoped_namespace,
        top_level_statements,
        global_attributes,
    };

    // Log some debug info
    trace!(
        "Parsed source file with {} using_directives, {} declarations, file_scoped_namespace: {}, {} top_level_statements",
        compilation_unit.using_directives.len(),
        compilation_unit.declarations.len(),
        compilation_unit.file_scoped_namespace.is_some(),
        compilation_unit.top_level_statements.len()
    );

    // Check if we fully consumed the input
    if log::log_enabled!(log::Level::Trace) {
        if !remaining.trim().is_empty() {
            trace!(
                "Warning: Parser did not consume all input. Remaining: {}",
                remaining.chars().take(80).collect::<String>()
            );
        } else {
            trace!("Successfully parsed the entire input.");
        }
    }

    Ok((remaining, compilation_unit))
}

// parse_using_directive moved to parser/declarations/using_directive_parser.rs

// Parse a top-level member (namespace or class)
fn parse_top_level_member(input: &str) -> BResult<&str, TopLevelDeclaration> {
    trace!(
        "Attempting to parse top-level member from: {}",
        input.chars().take(20).collect::<String>()
    );

    alt((
        // Try to parse namespace (block-scoped only, file-scoped is handled separately)
        map(parse_namespace_declaration, TopLevelDeclaration::Namespace),
        // Try to parse other type declarations
        map(parse_type_declaration, |type_decl| match type_decl {
            TypeDeclaration::Class(decl) => TopLevelDeclaration::Class(decl),
            TypeDeclaration::Struct(decl) => TopLevelDeclaration::Struct(decl),
            TypeDeclaration::Interface(decl) => TopLevelDeclaration::Interface(decl),
            TypeDeclaration::Record(decl) => TopLevelDeclaration::Record(decl),
            TypeDeclaration::Enum(decl) => TopLevelDeclaration::Enum(decl),
            TypeDeclaration::Delegate(decl) => TopLevelDeclaration::Delegate(decl),
        }),
    ))(input)
}
