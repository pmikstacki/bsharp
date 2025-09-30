// C# source file syntax - following Roslyn naming conventions
// Nodes in Roslyn use the "Syntax" suffix (e.g., NamespaceDeclarationSyntax)

use nom::branch::alt;
use nom::combinator::map;
use nom::Offset;

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
use crate::syntax::parser_helpers::with_recognized_span;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::TypeDeclaration;
use log::trace;
use crate::parser::SpanTable;
use crate::parser::preprocessor::parse_preprocessor_directive;
use crate::parser::helpers::directives::skip_preprocessor_directives;

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
    // Skip initial preprocessor directives
    let remaining = skip_preprocessor_directives(remaining, true);

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

    // Skip any leading preprocessor directives (treat as trivia)
    let remaining = skip_preprocessor_directives(remaining, true);

    // Parse global attributes first (assembly and module attributes)
    let (mut remaining, global_attributes) = parse_global_attributes(remaining)?;
    if !global_attributes.is_empty() {
        trace!(
            "Successfully parsed {} global attributes",
            global_attributes.len()
        );
    }

    // Parse using directives next (including optional 'global using' which we normalize to UsingDirective)
    let mut usings = Vec::new();
    loop {
        // Skip whitespace/comments between directives
        let (r, _) = ws(remaining)?;
        remaining = r;
        // Skip any preprocessor directives between using directives
        loop {
            if bpeek(crate::syntax::parser_helpers::bchar('#'))(remaining).is_ok() {
                if let Ok((rest, _dir)) = parse_preprocessor_directive(remaining) {
                    remaining = rest;
                    let (r2, _) = ws(remaining)?;
                    remaining = r2;
                    continue;
                }
            }
            break;
        }
        // Look for 'global using' or 'using'
        if bpeek(keyword("global"))(remaining).is_ok() {
            // consume 'global'
            let (r, _) = bws(keyword("global"))(remaining)?;
            // next must be 'using'
            if bpeek(keyword("using"))(r).is_ok() {
                // parse the rest of using directive by reusing parser on the remainder that starts after 'using'
                // We need a helper that expects we've already consumed 'using', but to keep it simple,
                // rebuild input by prefixing 'using ' back is cumbersome. Instead, call parse_using_directive starting at 'using'.
                // So we back up to a string that starts with 'using' by ignoring the consumed token above.
                // As a simpler approach, parse_using_directive from remaining after 'global' by expecting 'using' again.
                // We'll emulate by constructing a small closure.
                // Call parse_using_directive on original input (r) which still begins with 'using'.
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
            
            // After a file-scoped namespace, parse using directives and skip directives before members
            // 1) Skip any immediate preprocessor directives
            remaining = skip_preprocessor_directives(remaining, true);
            // 2) Collect using directives that appear after file-scoped namespace
            loop {
                let (r, _) = ws(remaining)?;
                remaining = r;
                if bpeek(keyword("using"))(remaining).is_ok() {
                    let (r_after, using_dir) = bws(parse_using_directive)(remaining)?;
                    usings.push(using_dir);
                    remaining = r_after;
                    continue;
                }
                // Allow global using after file-scoped namespace as well
                if bpeek(keyword("global"))(remaining).is_ok() {
                    let (r, _) = bws(keyword("global"))(remaining)?;
                    if bpeek(keyword("using"))(r).is_ok() {
                        let (r_after, using_dir) = parse_using_directive(r)?;
                        usings.push(using_dir);
                        remaining = r_after;
                        continue;
                    }
                }
                break;
            }
        }
    }

    // Now parse top-level members (namespaces, classes)
    let mut members = Vec::new();
    let mut top_level_statements = Vec::new();

    // Skip any whitespace between using directives and top-level members
    let (mut remaining, _) = ws(remaining)?;

    // If there's still input, try to parse top-level members or statements
    while !remaining.trim().is_empty() {
        // Skip any preprocessor directives between members/statements
        remaining = skip_preprocessor_directives(remaining, true);
        
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
            Err(e) => {
                // In strict mode, bubble the error immediately instead of attempting recovery
                if crate::parser::parse_mode::is_strict() {
                    return Err(e);
                } else {
                    // If we can't parse a member, try to parse top-level statements (C# 9+)
                    match parse_top_level_statements(remaining) {
                        Ok((rest, statements)) if !statements.is_empty() || rest != remaining => {
                            trace!(
                                "Parsed {} top-level statements (consumed: {})",
                                statements.len(), rest.len() != remaining.len()
                            );
                            top_level_statements.extend(statements);
                            remaining = rest;
                            break; // Parsed some statements; remaining content handled or empty
                        }
                        Ok((_rest, _statements)) => {
                            // Did not consume anything and found no statements; stop parsing loop gracefully
                            break;
                        }
                        Err(e2) => {
                            trace!("Failed to parse top-level member or statements: {:?}", e2);
                            break;
                        }
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

/// Strict entry point: require full consumption of input or return the original ErrorTree.
/// This is used by the CLI default path to ensure any syntax error leads to a failure.
pub fn parse_csharp_source_strict(input: &str) -> BResult<&str, CompilationUnit> {
    use crate::syntax::test_helpers::parse_all;
    parse_all(parse_csharp_source, input)
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

/// Parse a C# source file and collect byte-span ranges for top-level declarations.
/// Returns the CompilationUnit and a SpanTable keyed by a stable textual key.
pub fn parse_csharp_source_with_spans(
    input: &str,
) -> BResult<&str, (CompilationUnit, SpanTable)> {
    trace!(
        "Starting source file parsing (with spans) with input length: {}",
        input.len()
    );

    let mut span_table: SpanTable = SpanTable::new();

    // Skip any leading whitespace/comments and remove any BOM or other text markers
    let (remaining, _) = ws(input)?;

    // Parse global attributes first
    let (remaining, global_attributes) = parse_global_attributes(remaining)?;

    // Parse using directives
    let mut remaining = remaining;
    let mut usings = Vec::new();
    loop {
        let (r, _) = ws(remaining)?;
        remaining = r;
        // Skip any preprocessor directives between using directives
        loop {
            if bpeek(crate::syntax::parser_helpers::bchar('#'))(remaining).is_ok() {
                if let Ok((rest, _dir)) = parse_preprocessor_directive(remaining) {
                    remaining = rest;
                    let (r2, _) = ws(remaining)?;
                    remaining = r2;
                    continue;
                }
            }
            break;
        }
        if bpeek(keyword("global"))(remaining).is_ok() {
            let (r, _) = bws(keyword("global"))(remaining)?;
            if bpeek(keyword("using"))(r).is_ok() {
                let (r_after, using_dir) = parse_using_directive(r)?;
                usings.push(using_dir);
                remaining = r_after;
                continue;
            } else {
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

    // File-scoped namespace (collect span if present)
    let mut file_scoped_namespace = None;
    if bpeek(keyword("namespace"))(remaining).is_ok() {
        if let Ok((rest, (ns, range))) = with_recognized_span(input, parse_file_scoped_namespace_declaration)(remaining) {
            let key = format!("namespace::{}", ns.name.name);
            span_table.insert(key, range);
            file_scoped_namespace = Some(ns);
            remaining = rest;
        }
    }

    // Top-level members
    let mut members = Vec::new();
    let mut top_level_statements = Vec::new();

    let (mut remaining, _) = ws(remaining)?;
    while !remaining.trim().is_empty() {
        // Skip preprocessor directives between members/statements
        loop {
            let (r, _) = ws(remaining)?;
            remaining = r;
            if bpeek(crate::syntax::parser_helpers::bchar('#'))(remaining).is_ok() {
                if let Ok((rest, _dir)) = parse_preprocessor_directive(remaining) {
                    remaining = rest;
                    continue;
                }
            }
            break;
        }
        match with_recognized_span(input, parse_top_level_member)(remaining) {
            Ok((rest, (member, recognized_range))) => {
                // Compute file-scoped namespace prefix if present
                let ns_prefix = file_scoped_namespace
                    .as_ref()
                    .map(|fs| fs.name.name.as_str());
                let range_for_entry = recognized_range.clone();
                if let Some(key) = build_decl_key_with_prefix(&member, ns_prefix) {
                    span_table.insert(key, range_for_entry.clone());
                }
                // If this is a class, collect member-level spans by reparsing its body with a span-aware member parser
                if let TopLevelDeclaration::Class(class_decl) = &member {
                    let class_slice = &input[range_for_entry.clone()];
                    collect_class_member_spans(
                        input,
                        class_slice,
                        ns_prefix,
                        &class_decl.name.name,
                        &mut span_table,
                    );
                }
                members.push(member);
                remaining = rest;
                let (after_ws, _) = ws(remaining)?;
                remaining = after_ws;
            }
            Err(e) => {
                if crate::parser::parse_mode::is_strict() {
                    return Err(e);
                } else {
                    match parse_top_level_statements(remaining) {
                        Ok((rest, statements)) => {
                            top_level_statements.extend(statements);
                            remaining = rest;
                            break;
                        }
                        Err(e2) => {
                            trace!("Failed to parse top-level member or statements (with spans): {:?}", e2);
                            break;
                        }
                    }
                }
            }
        }
    }

    let compilation_unit = CompilationUnit {
        using_directives: usings,
        declarations: members,
        file_scoped_namespace,
        top_level_statements,
        global_attributes,
    };

    Ok((remaining, (compilation_unit, span_table)))
}

/// Build a stable textual key for a top-level declaration to store span ranges,
/// optionally prefixing type declarations with the file-scoped namespace name.
fn build_decl_key_with_prefix(
    decl: &TopLevelDeclaration,
    file_scoped_ns: Option<&str>,
) -> Option<String> {
    let prefixed = |kind: &str, name: &str| -> String {
        match file_scoped_ns {
            Some(ns) => format!("{}::{}::{}", kind, ns, name),
            None => format!("{}::{}", kind, name),
        }
    };

    match decl {
        TopLevelDeclaration::Namespace(ns) => Some(format!("namespace::{}", ns.name.name)),
        TopLevelDeclaration::Class(c) => Some(prefixed("class", &c.name.name)),
        TopLevelDeclaration::Struct(s) => Some(prefixed("struct", &s.name.name)),
        TopLevelDeclaration::Record(r) => Some(prefixed("record", &r.name.name)),
        TopLevelDeclaration::Interface(i) => Some(prefixed("interface", &i.name.name)),
        TopLevelDeclaration::Enum(e) => Some(prefixed("enum", &e.name.name)),
        TopLevelDeclaration::Delegate(d) => Some(prefixed("delegate", &d.name.name)),
        TopLevelDeclaration::FileScopedNamespace(fs) => Some(format!("namespace::{}", fs.name.name)),
        TopLevelDeclaration::GlobalAttribute(_) => None,
    }
}

/// Temporary no-op: member-level span collection will be implemented in Milestone B.
fn collect_class_member_spans(
    whole: &str,
    class_slice: &str,
    file_scoped_ns: Option<&str>,
    class_name: &str,
    spans: &mut SpanTable,
) {
    // Find the class body start
    let Some(brace_idx) = class_slice.find('{') else { return; };
    let body_input = &class_slice[brace_idx..];

    // Owner prefix for member keys
    let owner_prefix = match file_scoped_ns {
        Some(ns) => format!("{}::{}", ns, class_name),
        None => class_name.to_string(),
    };

    // Manually iterate class members
    use crate::parser::expressions::declarations::type_declaration_parser::parse_class_member_for_spans as parse_member;
    use crate::syntax::comment_parser::parse_whitespace_or_comments as ws_comments;
    use crate::syntax::nodes::declarations::ClassBodyDeclaration as CBD;

    // Skip the opening '{'
    let mut cur = if body_input.starts_with('{') { &body_input[1..] } else { body_input };
    loop {
        // Consume whitespace/comments
        if let Ok((after, _)) = ws_comments(cur) {
            cur = after;
        }
        // Stop at closing '}'
        let ahead = cur.trim_start();
        if ahead.starts_with('}') || cur.is_empty() {
            break;
        }

        match parse_member(cur) {
            Ok((rest, member)) => {
                let start = whole.offset(cur);
                let end = whole.offset(rest);
                let range = start..end;
                match &member {
                    CBD::Method(m) => {
                        let key = format!("method::{}::{}", owner_prefix, m.name.name);
                        spans.insert(key, range);
                    }
                    CBD::Constructor(_) => {
                        let key = format!("ctor::{}", owner_prefix);
                        spans.insert(key, range);
                    }
                    _ => {}
                }
                cur = rest;
            }
            Err(_) => {
                // Unable to parse further safely; stop
                break;
            }
        }
    }
}
