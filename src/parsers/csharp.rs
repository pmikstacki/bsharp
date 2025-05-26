// C# source file parser - following Roslyn naming conventions
// Nodes in Roslyn use the "Syntax" suffix (e.g., NamespaceDeclarationSyntax)

use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::map;

use crate::parser::ast::{CompilationUnit, TopLevelDeclaration};
use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::TypeDeclaration;
use crate::parser::nodes::declarations::UsingDirective;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bchar, bws, keyword};
use crate::parsers::declarations::namespace_declaration_parser::parse_namespace_declaration;
use crate::parsers::declarations::file_scoped_namespace_parser::parse_file_scoped_namespace_declaration;
use crate::parsers::declarations::type_declaration_parser::parse_type_declaration;
use crate::parsers::declarations::global_attribute_parser::parse_global_attributes;
use crate::parsers::identifier_parser::parse_qualified_name;
use crate::parsers::statements::top_level_statement_parser::parse_top_level_statements;

/// Parse a C# source file following Roslyn's model where a source file contains:
/// - global attributes (assembly/module attributes)
/// - using directives (imports)
/// - optional file-scoped namespace (C# 10+)
/// - namespace or top-level type declarations
/// - optional top-level statements (C# 9+)
pub fn parse_csharp_source(input: &str) -> BResult<&str, CompilationUnit> {
    println!("Starting source file parsing with input length: {}", input.len());
    
    // Skip any leading whitespace and remove any BOM or other text markers
    let (remaining, _) = multispace0(input)?;
    
    // Trace the input start
    let preview = remaining.chars().take(40).collect::<String>();
    println!("Parsing file starting with: {}", preview);
    
    // Split the input into chunks for better debugging
    let input_lines: Vec<&str> = input.lines().collect();
    println!("Input has {} lines", input_lines.len());
    for (i, line) in input_lines.iter().take(5).enumerate() {
        println!("Line {}: {}", i, line);
    }
    
    // Parse global attributes first (assembly and module attributes)
    let (remaining, global_attributes) = parse_global_attributes(remaining)?;
    if !global_attributes.is_empty() {
        println!("Successfully parsed {} global attributes", global_attributes.len());
    }
    
    // Parse using directives next (global using directives before namespace)
    let mut usings = Vec::new();
    let mut remaining = remaining;
    
    // First check if the line starts with 'using'
    if remaining.trim_start().starts_with("using") {
        println!("Found using directive at start of file");
        
        // Keep parsing using directives until we can't find any more
        loop {
            // Skip any whitespace before the using directive
            let (after_ws, _) = multispace0(remaining)?;
            
            // If we don't find 'using' after whitespace, we're done with using directives
            if !after_ws.starts_with("using") {
                println!("No more using directives found");
                remaining = after_ws;
                break;
            }
            
            // Try to parse a single using directive
            match parse_using_directive(after_ws) {
                Ok((rest, using_directive)) => {
                    println!("Successfully parsed using directive");
                    usings.push(using_directive);
                    remaining = rest;
                },
                Err(e) => {
                    println!("Error parsing using directive: {:?}", e);
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
                println!("Found file-scoped namespace pattern");
                
                match parse_file_scoped_namespace_declaration(remaining) {
                    Ok((rest, namespace)) => {
                        println!("Successfully parsed file-scoped namespace: {}", namespace.name.name);
                        file_scoped_namespace = Some(namespace);
                        remaining = rest;
                    },
                    Err(e) => {
                        println!("Error parsing file-scoped namespace: {:?}", e);
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
    let (mut remaining, _) = multispace0(remaining)?;
    
    // If there's still input, try to parse top-level members or statements
    while !remaining.trim().is_empty() {
        // First try to parse top-level members (classes, namespaces, etc.)
        match parse_top_level_member(remaining) {
            Ok((rest, member)) => {
                println!("Successfully parsed top-level member");
                members.push(member);
                remaining = rest;
                
                // Skip any whitespace between members
                let (after_ws, _) = multispace0(remaining)?;
                remaining = after_ws;
            },
            Err(_) => {
                // If we can't parse a member, try to parse top-level statements (C# 9+)
                match parse_top_level_statements(remaining) {
                    Ok((rest, statements)) => {
                        println!("Successfully parsed {} top-level statements", statements.len());
                        top_level_statements.extend(statements);
                        remaining = rest;
                        break; // Top-level statements consume the rest of the file
                    },
                    Err(e) => {
                        println!("Failed to parse top-level member or statements: {:?}", e);
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
    println!("Parsed source file with {} using_directives, {} declarations, file_scoped_namespace: {}, {} top_level_statements", 
        compilation_unit.using_directives.len(), 
        compilation_unit.declarations.len(),
        compilation_unit.file_scoped_namespace.is_some(),
        compilation_unit.top_level_statements.len());
    
    // Check if we fully consumed the input
    if !remaining.trim().is_empty() {
        println!("Warning: Parser did not consume all input. Remaining: {}", 
            remaining.chars().take(40).collect::<String>());
    } else {
        println!("Successfully parsed the entire input.");
    }
    
    Ok((remaining, compilation_unit))
}

// Parse a using directive
fn parse_using_directive(input: &str) -> BResult<&str, UsingDirective> {
    // Log the input for debugging
    println!("Attempting to parse using directive from: {}", input.chars().take(40).collect::<String>());
    
    // Using more explicit steps to parse a using directive
    let (input, _) = multispace0(input)?;  // Skip leading whitespace
    
    // Parse the 'using' keyword
    let (input, _) = keyword("using")(input)?;
    println!("Found 'using' keyword");
    
    // Parse whitespace after 'using'
    let (input, _) = bws(multispace0)(input)?;
    
    // Parse the namespace (qualified name)
    let (input, namespace) = parse_qualified_name(input)?;
    
    // Create the namespace string
    let ns_str = namespace.iter().map(|id| id.name.clone()).collect::<Vec<_>>().join(".");
    println!("Found namespace: {}", ns_str);
    
    // Parse the semicolon
    let (input, _) = bws(bchar(';'))(input)?;
    println!("Found semicolon");
    
    // Parse any trailing whitespace
    let (input, _) = multispace0(input)?;
    
    // Create the UsingDirective
    let using_directive = UsingDirective::Namespace { 
        namespace: Identifier { name: ns_str.clone() } 
    };
    
    println!("Successfully parsed using directive: {}", ns_str);
    Ok((input, using_directive))
}

// Parse a top-level member (namespace or class)
fn parse_top_level_member(input: &str) -> BResult<&str, TopLevelDeclaration> {
    println!("Attempting to parse top-level member from: {}", input.chars().take(20).collect::<String>());
    
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
