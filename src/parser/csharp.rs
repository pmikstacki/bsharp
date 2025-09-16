// C# source file syntax - following Roslyn naming conventions
// Nodes in Roslyn use the "Syntax" suffix (e.g., NamespaceDeclarationSyntax)

use nom::branch::alt;
// use nom::character::complete::multispace0; // replaced by comment-aware ws
use nom::combinator::map;

use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::TypeDeclaration;
// parser_helpers imported selectively in sub-parser; this module only needs ws
use crate::syntax::comment_parser::ws;
use crate::parser::declarations::using_directive_parser::parse_using_directive;
use crate::parser::declarations::namespace_declaration_parser::parse_namespace_declaration;
use crate::parser::declarations::file_scoped_namespace_parser::parse_file_scoped_namespace_declaration;
use crate::parser::declarations::type_declaration_parser::parse_type_declaration;
use crate::parser::declarations::global_attribute_parser::parse_global_attributes;
use crate::parser::statements::top_level_statement_parser::parse_top_level_statements;
use log::trace;

/// Parse a C# source file following Roslyn's model where a source file contains:
/// - global attributes (assembly/module attributes)
/// - using directives (imports)
/// - optional file-scoped namespace (C# 10+)
/// - namespace or top-level type declarations
/// - optional top-level statements (C# 9+)
pub fn parse_csharp_source(input: &str) -> BResult<&str, CompilationUnit> {
    trace!("Starting source file parsing with input length: {}", input.len());
    
    // Skip any leading whitespace/comments and remove any BOM or other text markers
    let (remaining, _) = ws(input)?;
    
    // Trace the input start
    let preview = remaining.chars().take(40).collect::<String>();
    trace!("Parsing file starting with: {}", preview);
    
    // Split the input into chunks for better debugging
    let input_lines: Vec<&str> = input.lines().collect();
    trace!("Input has {} lines", input_lines.len());
    for (i, line) in input_lines.iter().take(5).enumerate() {
        trace!("Line {}: {}", i, line);
    }
    
    // Parse global attributes first (assembly and module attributes)
    let (remaining, global_attributes) = parse_global_attributes(remaining)?;
    if !global_attributes.is_empty() {
        trace!("Successfully parsed {} global attributes", global_attributes.len());
    }
    
    // Parse using directives next (global using directives before namespace)
    let mut usings = Vec::new();
    let mut remaining = remaining;
    
    // First check if the line starts with 'using'
    if remaining.trim_start().starts_with("using") {
        trace!("Found using directive at start of file");
        
        // Keep parsing using directives until we can't find any more
        loop {
            // Skip any whitespace before the using directive
            let (after_ws, _) = ws(remaining)?;
            
            // If we don't find 'using' after whitespace, we're done with using directives
            if !after_ws.starts_with("using") {
                trace!("No more using directives found");
                remaining = after_ws;
                break;
            }
            
            // Try to parse a single using directive
            match parse_using_directive(after_ws) {
                Ok((rest, using_directive)) => {
                    trace!("Successfully parsed using directive");
                    usings.push(using_directive);
                    remaining = rest;
                },
                Err(e) => {
                    trace!("Error parsing using directive: {:?}", e);
                    break;
                }
            }
        }
    }
    
    // Try to parse file-scoped namespace (C# 10+)
    let mut file_scoped_namespace = None;
    
    // Better detection: check if we have "namespace identifier;" pattern at the start
    if remaining.trim_start().starts_with("namespace") {
        // Look ahead to see if this looks like a file-scoped namespace
        let test_input = remaining.trim_start();
        let lines: Vec<&str> = test_input.lines().collect();
        if !lines.is_empty() {
            let first_line = lines[0].trim();
            if first_line.starts_with("namespace") && first_line.ends_with(";") {
                trace!("Found file-scoped namespace pattern");
                
                match parse_file_scoped_namespace_declaration(remaining) {
                    Ok((rest, namespace)) => {
                        trace!("Successfully parsed file-scoped namespace: {}", namespace.name.name);
                        file_scoped_namespace = Some(namespace);
                        remaining = rest;
                    },
                    Err(e) => {
                        trace!("Error parsing file-scoped namespace: {:?}", e);
                        // Continue parsing as regular namespace
                    }
                }
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
        // First try to parse top-level members (classes, namespaces, etc.)
        match parse_top_level_member(remaining) {
            Ok((rest, member)) => {
                trace!("Successfully parsed top-level member");
                members.push(member);
                remaining = rest;
                
                // Skip any whitespace between members
                let (after_ws, _) = ws(remaining)?;
                remaining = after_ws;
            },
            Err(_) => {
                // If we can't parse a member, try to parse top-level statements (C# 9+)
                match parse_top_level_statements(remaining) {
                    Ok((rest, statements)) => {
                        trace!("Successfully parsed {} top-level statements", statements.len());
                        top_level_statements.extend(statements);
                        remaining = rest;
                        break; // Top-level statements consume the rest of the file
                    },
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
    trace!("Parsed source file with {} using_directives, {} declarations, file_scoped_namespace: {}, {} top_level_statements", 
        compilation_unit.using_directives.len(), 
        compilation_unit.declarations.len(),
        compilation_unit.file_scoped_namespace.is_some(),
        compilation_unit.top_level_statements.len());
    
    // Check if we fully consumed the input
    if !remaining.trim().is_empty() {
        trace!("Warning: Parser did not consume all input. Remaining: {}", 
            remaining.chars().take(40).collect::<String>());
    } else {
        trace!("Successfully parsed the entire input.");
    }
    
    Ok((remaining, compilation_unit))
}

// parse_using_directive moved to parser/declarations/using_directive_parser.rs

// Parse a top-level member (namespace or class)
fn parse_top_level_member(input: &str) -> BResult<&str, TopLevelDeclaration> {
    trace!("Attempting to parse top-level member from: {}", input.chars().take(20).collect::<String>());
    
    alt((
        // Try to parse namespace (block-scoped only, file-scoped is handled separately)
        map(parse_namespace_declaration, TopLevelDeclaration::Namespace),
        // Try to parse other type declarations
        map(parse_type_declaration, |type_decl| {
            match type_decl {
                TypeDeclaration::Class(decl) => TopLevelDeclaration::Class(decl),
                TypeDeclaration::Struct(decl) => TopLevelDeclaration::Struct(decl),
                TypeDeclaration::Interface(decl) => TopLevelDeclaration::Interface(decl),
                TypeDeclaration::Record(decl) => TopLevelDeclaration::Record(decl),
                TypeDeclaration::Enum(decl) => TopLevelDeclaration::Enum(decl),
                TypeDeclaration::Delegate(decl) => TopLevelDeclaration::Delegate(decl),
            }
        }),
    ))(input)
}
