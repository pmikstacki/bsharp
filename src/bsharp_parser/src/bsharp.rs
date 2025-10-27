// C# source file syntax - following Roslyn naming conventions
// Nodes in Roslyn use the "Syntax" suffix (e.g., NamespaceDeclarationSyntax)

use nom::Offset;

use crate::parser::expressions::declarations::file_scoped_namespace_parser::parse_file_scoped_namespace_declaration;
use crate::parser::expressions::declarations::global_attribute_parser::parse_global_attributes;
use crate::parser::expressions::declarations::namespace_declaration_parser::parse_namespace_declaration;
use crate::parser::expressions::declarations::type_declaration_parser::parse_type_declaration;
use crate::parser::expressions::declarations::using_directive_parser::parse_using_directive;
use crate::parser::expressions::statements::top_level_statement_parser::parse_top_level_statements;
use crate::parser::helpers::directives::skip_preprocessor_directives;
use crate::parser::keywords::contextual_misc_keywords::kw_global;
use crate::parser::keywords::declaration_keywords::{kw_namespace, kw_using};
// parser_helpers imported selectively in sub-parser; this module only needs ws
use crate::parser::SpanTable;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use syntax::span::Span;

use log::trace;
use nom::Parser;
use nom::combinator::peek;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
use syntax::Identifier as SynIdentifier;
use syntax::ast::{CompilationUnit, TopLevelDeclaration};
use syntax::declarations::{ClassBodyDeclaration, GlobalUsingDirective, TypeDeclaration};

fn ident_to_string(id: &SynIdentifier) -> String {
    match id {
        SynIdentifier::Simple(s) => s.clone(),
        SynIdentifier::QualifiedIdentifier(segs) => segs.join("."),
        SynIdentifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
    }
}

/// Parse a C# source file following Roslyn's model where a source file contains:
/// - global attributes (assembly/module attributes)
/// - using directives (imports)
/// - optional file-scoped namespace (C# 10+)
/// - namespace or top-level type declarations
/// - optional top-level statements (C# 9+)
pub fn parse_csharp_source<'a, S>(input: S) -> BResult<'a, CompilationUnit>
where
    S: Into<Span<'a>>,
{
    let input: Span = input.into();
    trace!(
        "Starting source file parsing with input length: {}",
        input.fragment().len()
    );

    // Skip any leading whitespace/comments
    let (mut remaining, _) = ws(input)?;
    // Skip initial preprocessor directives
    remaining = skip_preprocessor_directives(remaining, true);

    if log::log_enabled!(log::Level::Trace) {
        // Trace the input start (debug-only)
        let preview = remaining.fragment().chars().take(80).collect::<String>();
        trace!("Parsing file starting with: {}", preview);
        let input_lines: Vec<&str> = input.fragment().lines().collect();
        trace!("Input has {} lines", input_lines.len());
        for (i, line) in input_lines.iter().take(5).enumerate() {
            trace!("Line {}: {}", i, line);
        }
    }

    // Skip any leading preprocessor directives (treat as trivia)
    remaining = skip_preprocessor_directives(remaining, true);

    // Parse global attributes first (assembly and module attributes)
    let (mut remaining, global_attributes) = parse_global_attributes(remaining)?;
    if !global_attributes.is_empty() {
        trace!(
            "Successfully parsed {} global attributes",
            global_attributes.len()
        );
    }

    // Parse using directives next; keep global usings separately
    let mut usings = Vec::new();
    let mut global_usings: Vec<GlobalUsingDirective> = Vec::new();
    loop {
        // Skip whitespace/comments between directives
        let (r, _) = ws(remaining)?;
        remaining = r;
        // Skip any preprocessor directives between using directives
        remaining = skip_preprocessor_directives(remaining, true);
        // Look for 'global using' or 'using'
        if peek(delimited(ws, kw_global(), ws))
            .parse(remaining)
            .is_ok()
        {
            let (r_after_global, _) = delimited(ws, kw_global(), ws).parse(remaining)?;
            if peek(delimited(ws, kw_using(), ws))
                .parse(r_after_global)
                .is_ok()
            {
                let (r_after, using_dir) = parse_using_directive(r_after_global)?;
                global_usings.push(GlobalUsingDirective {
                    using_directive: using_dir,
                });
                remaining = r_after;
                continue;
            } else {
                break;
            }
        } else if peek(delimited(ws, kw_using(), ws)).parse(remaining).is_ok() {
            let (r_after, using_dir) = parse_using_directive(remaining)?;
            usings.push(using_dir);
            remaining = r_after;
            continue;
        } else {
            break;
        }
    }
    if log::log_enabled!(log::Level::Trace) && !usings.is_empty() {
        trace!("Successfully parsed {} using directives", usings.len());
    }

    // Try to parse file-scoped namespace (C# 10+)
    let mut file_scoped_namespace = None;

    // Prefer lookahead-based detection rather than string peeks
    if peek(delimited(ws, kw_namespace(), ws))
        .parse(remaining)
        .is_ok()
    {
        match parse_file_scoped_namespace_declaration(remaining) {
            Ok((rest, namespace)) => {
                if log::log_enabled!(log::Level::Trace) {
                    trace!(
                        "Successfully parsed file-scoped namespace: {}",
                        ident_to_string(&namespace.name)
                    );
                }
                file_scoped_namespace = Some(namespace);
                remaining = rest;

                // After a file-scoped namespace, parse using directives and skip directives before members
                // 1) Skip any immediate preprocessor directives
                remaining = skip_preprocessor_directives(remaining, true);
                // 2) Collect using directives (global and non-global) that appear after file-scoped namespace
                loop {
                    let (r, _) = ws(remaining)?;
                    remaining = r;
                    if peek(delimited(ws, kw_using(), ws)).parse(remaining).is_ok() {
                        let (r_after, using_dir) = parse_using_directive(remaining)?;
                        usings.push(using_dir);
                        remaining = r_after;
                        continue;
                    }
                    // Allow global using after file-scoped namespace as well
                    if peek(delimited(ws, kw_global(), ws))
                        .parse(remaining)
                        .is_ok()
                    {
                        let (r_after_global, _) =
                            delimited(ws, kw_global(), ws).parse(remaining)?;
                        if peek(delimited(ws, kw_using(), ws))
                            .parse(r_after_global)
                            .is_ok()
                        {
                            let (r_after, using_dir) = parse_using_directive(r_after_global)?;
                            global_usings.push(GlobalUsingDirective {
                                using_directive: using_dir,
                            });
                            remaining = r_after;
                            continue;
                        }
                    }
                    break;
                }
            }
            Err(_e) => {
                // Not a file-scoped namespace; continue and allow block-scoped parsing later.
            }
        }
    }

    // Now parse top-level members (namespaces, classes)
    let mut members = Vec::new();
    let mut top_level_statements = Vec::new();

    // Skip any whitespace between using directives and top-level members
    let (mut remaining, _) = ws(remaining)?;

    // If there's still input, try to parse top-level members or statements
    'outer: while !remaining.fragment().trim().is_empty() {
        let mut progressed = false;

        let before_skip = remaining.fragment().len();
        remaining = skip_preprocessor_directives(remaining, true);
        progressed |= remaining.fragment().len() != before_skip;

        let parsed_entry = match parse_top_level_member(remaining) {
            Ok((rest, member)) => {
                trace!("Successfully parsed top-level member");
                members.push(member);
                remaining = rest;
                true
            }
            Err(member_err) => match parse_top_level_statements(remaining) {
                Ok((rest, statements)) => {
                    let consumed = rest.location_offset() > remaining.location_offset();
                    if !statements.is_empty() {
                        trace!("Parsed {} top-level statements", statements.len());
                        top_level_statements.extend(statements);
                        remaining = rest;
                        true
                    } else if consumed {
                        remaining = rest;
                        true
                    } else if crate::parser::parse_mode::is_strict() {
                        return Err(member_err);
                    } else {
                        break 'outer;
                    }
                }
                Err(stmt_err) => {
                    if crate::parser::parse_mode::is_strict() {
                        return Err(stmt_err);
                    } else {
                        trace!(
                            "Failed to parse top-level member or statements: {:?}",
                            stmt_err
                        );
                        break 'outer;
                    }
                }
            },
        };

        if parsed_entry {
            progressed = true;
            let before_ws = remaining.location_offset();
            let (after_ws, _) = ws(remaining)?;
            progressed |= after_ws.location_offset() != before_ws;
            remaining = after_ws;
        }

        if !progressed {
            break;
        }
    }

    // Create the source file
    let compilation_unit = CompilationUnit {
        using_directives: usings,
        global_using_directives: global_usings,
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
pub fn parse_csharp_source_strict(input: Span) -> BResult<CompilationUnit> {
    let (rest, unit) = parse_csharp_source(input)?;
    if rest.fragment().trim().is_empty() {
        Ok((rest, unit))
    } else {
        let err = ErrorTree::Base {
            location: rest,
            kind: BaseErrorKind::Expected(Expectation::Eof),
        };
        Err(nom::Err::Error(err))
    }
}

// parse_using_directive moved to parser/declarations/using_directive_parser.rs

// Parse a top-level member (namespace or class)
fn type_decl_to_top_level(d: TypeDeclaration) -> TopLevelDeclaration {
    match d {
        TypeDeclaration::Class(decl) => TopLevelDeclaration::Class(decl),
        TypeDeclaration::Struct(decl) => TopLevelDeclaration::Struct(decl),
        TypeDeclaration::Interface(decl) => TopLevelDeclaration::Interface(decl),
        TypeDeclaration::Record(decl) => TopLevelDeclaration::Record(decl),
        TypeDeclaration::Enum(decl) => TopLevelDeclaration::Enum(decl),
        TypeDeclaration::Delegate(decl) => TopLevelDeclaration::Delegate(decl),
    }
}

pub(crate) fn parse_top_level_member(input: Span) -> BResult<TopLevelDeclaration> {
    trace!(
        "Attempting to parse top-level member from: {}",
        input.fragment().chars().take(20).collect::<String>()
    );
    if let Ok((rest, ns)) = parse_namespace_declaration(input) {
        return Ok((rest, TopLevelDeclaration::Namespace(ns)));
    }
    let (rest, td) = parse_type_declaration(input)?;
    Ok((rest, type_decl_to_top_level(td)))
}

/// Parse a C# source file and collect byte-span ranges for top-level declarations.
/// Returns the CompilationUnit and a SpanTable keyed by a stable textual key.
pub fn parse_csharp_source_with_spans<'a, S>(input: S) -> BResult<'a, (CompilationUnit, SpanTable)>
where
    S: Into<Span<'a>>,
{
    let input: Span = input.into();
    trace!(
        "Starting source file parsing (with spans) with input length: {}",
        input.fragment().len()
    );

    let mut span_table: SpanTable = SpanTable::new();
    let _original = input.fragment();

    // Skip any leading whitespace/comments and remove any BOM or other text markers
    let (remaining, _) = ws(input)?;

    // Parse global attributes first
    let (remaining, global_attributes) = parse_global_attributes(remaining)?;

    // Parse using directives (track global usings separately)
    let mut remaining = remaining;
    let mut usings = Vec::new();
    let mut global_usings: Vec<GlobalUsingDirective> = Vec::new();
    loop {
        let (r, _) = ws(remaining)?;
        remaining = r;
        // Skip any preprocessor directives between using directives
        remaining = skip_preprocessor_directives(remaining, true);
        if peek(delimited(ws, kw_global(), ws))
            .parse(remaining)
            .is_ok()
        {
            let (r_after_global, _) = delimited(ws, kw_global(), ws).parse(remaining)?;
            if peek(delimited(ws, kw_using(), ws))
                .parse(r_after_global)
                .is_ok()
            {
                let (r_after, using_dir) = parse_using_directive(r_after_global)?;
                global_usings.push(GlobalUsingDirective {
                    using_directive: using_dir,
                });
                remaining = r_after;
                continue;
            } else {
                break;
            }
        } else if peek(delimited(ws, kw_using(), ws)).parse(remaining).is_ok() {
            let (r_after, using_dir) = parse_using_directive(remaining)?;
            usings.push(using_dir);
            remaining = r_after;
            continue;
        } else {
            break;
        }
    }

    // File-scoped namespace (collect span if present)
    let mut file_scoped_namespace = None;
    if peek(delimited(ws, kw_namespace(), ws))
        .parse(remaining)
        .is_ok()
    {
        if let Ok((rest, (recognized, ns))) = (|i| parse_file_scoped_namespace_declaration(i))
            .with_recognized()
            .parse(remaining)
        {
            let start = recognized.location_offset();
            let end = start + recognized.fragment().len();
            let key = format!("namespace::{}", ident_to_string(&ns.name));
            span_table.insert(key, start..end);
            file_scoped_namespace = Some(ns);
            remaining = rest;
        }
    }

    // Top-level members
    let mut members = Vec::new();
    let mut top_level_statements = Vec::new();

    let (mut remaining, _) = ws(remaining)?;
    'outer: while !remaining.trim().is_empty() {
        let mut progressed = false;

        let before_ws = remaining.location_offset();
        let (r, _) = ws(remaining)?;
        progressed |= r.location_offset() != before_ws;
        remaining = r;
        remaining = skip_preprocessor_directives(remaining, true);

        let parsed_entry = match (|i| parse_top_level_member(i))
            .with_recognized()
            .parse(remaining)
        {
            Ok((rest, (recognized, member))) => {
                let ns_prefix_string = file_scoped_namespace
                    .as_ref()
                    .map(|fs| ident_to_string(&fs.name));
                let ns_prefix = ns_prefix_string.as_deref();
                let start = recognized.location_offset();
                let end = start + recognized.fragment().len();
                let range_for_entry = start..end;
                if let Some(key) = build_decl_key_with_prefix(&member, ns_prefix) {
                    span_table.insert(key, range_for_entry.clone());
                }
                if let TopLevelDeclaration::Class(class_decl) = &member {
                    let class_slice = &input.fragment()[range_for_entry.clone()];
                    collect_class_member_spans(
                        input.fragment(),
                        class_slice,
                        range_for_entry.start,
                        ns_prefix,
                        &ident_to_string(&class_decl.name),
                        &mut span_table,
                    );
                }
                members.push(member);
                remaining = rest;
                true
            }
            Err(member_err) => match parse_top_level_statements(remaining) {
                Ok((rest, statements)) => {
                    let consumed = rest.location_offset() > remaining.location_offset();
                    if !statements.is_empty() {
                        top_level_statements.extend(statements);
                        remaining = rest;
                        true
                    } else if consumed {
                        remaining = rest;
                        true
                    } else if crate::parser::parse_mode::is_strict() {
                        return Err(member_err);
                    } else {
                        break 'outer;
                    }
                }
                Err(stmt_err) => {
                    if crate::parser::parse_mode::is_strict() {
                        return Err(stmt_err);
                    } else {
                        trace!(
                            "Failed to parse top-level member or statements (with spans): {:?}",
                            stmt_err
                        );
                        break 'outer;
                    }
                }
            },
        };

        if parsed_entry {
            progressed = true;
            let before_ws = remaining.location_offset();
            let (after_ws, _) = ws(remaining)?;
            progressed |= after_ws.location_offset() != before_ws;
            remaining = after_ws;
        }

        if !progressed {
            break;
        }
    }

    let compilation_unit = CompilationUnit {
        using_directives: usings,
        global_using_directives: global_usings,
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
        TopLevelDeclaration::Namespace(ns) => {
            Some(format!("namespace::{}", ident_to_string(&ns.name)))
        }
        TopLevelDeclaration::Class(c) => Some(prefixed("class", &ident_to_string(&c.name))),
        TopLevelDeclaration::Struct(s) => Some(prefixed("struct", &ident_to_string(&s.name))),
        TopLevelDeclaration::Record(r) => Some(prefixed("record", &ident_to_string(&r.name))),
        TopLevelDeclaration::Interface(i) => Some(prefixed("interface", &ident_to_string(&i.name))),
        TopLevelDeclaration::Enum(e) => Some(prefixed("enum", &ident_to_string(&e.name))),
        TopLevelDeclaration::Delegate(d) => Some(prefixed("delegate", &ident_to_string(&d.name))),
        TopLevelDeclaration::FileScopedNamespace(fs) => {
            Some(format!("namespace::{}", ident_to_string(&fs.name)))
        }
        TopLevelDeclaration::GlobalAttribute(_) => None,
    }
}

/// Temporary no-op: member-level span collection will be implemented in Milestone B.
fn collect_class_member_spans(
    _whole: &str,
    class_slice: &str,
    class_abs_start: usize,
    file_scoped_ns: Option<&str>,
    class_name: &str,
    spans: &mut SpanTable,
) {
    // Find the class body start
    let Some(brace_idx) = class_slice.find('{') else {
        return;
    };
    let body_input = &class_slice[brace_idx..];

    // Owner prefix for member keys
    let owner_prefix = match file_scoped_ns {
        Some(ns) => format!("{}::{}", ns, class_name),
        None => class_name.to_string(),
    };

    // Manually iterate class members
    use crate::parser::expressions::declarations::type_declaration_parser::parse_class_member_for_spans as parse_member;
    use crate::trivia::comment_parser::parse_whitespace_or_comments as ws_comments;

    // Skip the opening '{'
    let mut cur = body_input.strip_prefix('{').unwrap_or(body_input);
    loop {
        // Consume whitespace/comments
        if let Ok((after, _)) = ws_comments(cur.into()) {
            cur = after.fragment();
        }
        // Stop at closing '}'
        let ahead = cur.trim_start();
        if ahead.starts_with('}') || cur.is_empty() {
            break;
        }

        match parse_member(cur.into()) {
            Ok((rest, member)) => {
                // Compute offsets relative to class_slice, then add class_abs_start for absolute positions
                let rel_start = class_slice.offset(cur);
                let rel_end = class_slice.offset(rest.fragment());
                let start = class_abs_start + rel_start;
                let end = class_abs_start + rel_end;
                let range = start..end;
                match &member {
                    ClassBodyDeclaration::Method(m) => {
                        let key = format!("method::{}::{}", owner_prefix, ident_to_string(&m.name));
                        spans.insert(key, range);
                    }
                    ClassBodyDeclaration::Constructor(_) => {
                        let key = format!("ctor::{}", owner_prefix);
                        spans.insert(key, range);
                    }
                    ClassBodyDeclaration::Property(p) => {
                        let key =
                            format!("property::{}::{}", owner_prefix, ident_to_string(&p.name));
                        spans.insert(key, range);
                    }
                    _ => {}
                }
                cur = rest.fragment();
            }
            Err(_) => {
                // Unable to parse further safely; stop
                break;
            }
        }
    }
}
