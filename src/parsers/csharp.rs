// C# source file parser - following Roslyn naming conventions
// Nodes in Roslyn use the "Syntax" suffix (e.g., NamespaceDeclarationSyntax)

use nom::branch::alt;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::map;

use crate::parser::errors::BResult;
use crate::parser::ast::{CompilationUnit, TopLevelMember};
use crate::parser::nodes::declarations::UsingDirective;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bchar, keyword, nom_to_bs, bws};
use crate::parsers::declarations::class_declaration_parser::parse_class_declaration;
use crate::parsers::declarations::struct_declaration_parser::parse_struct_declaration;
use crate::parsers::declarations::record_declaration_parser::parse_record_declaration;
use crate::parsers::declarations::interface_declaration_parser::parse_interface_declaration;
use crate::parsers::declarations::enum_declaration_parser::parse_enum_declaration;
use crate::parsers::declarations::namespace_declaration_parser::parse_namespace_declaration;
use crate::parsers::identifier_parser::parse_qualified_name;

/// Parse a C# source file following Roslyn's model where a source file contains:
/// - using directives (imports)
/// - namespace or top-level type declarations
pub fn parse_csharp_source<'a>(input: &'a str) -> BResult<&'a str, CompilationUnit<'a>> {
    println!("Starting source file parsing with input length: {}", input.len());
    
    // Skip any leading whitespace and remove any BOM or other text markers
    let (mut remaining, _) = multispace0(input)?;
    
    // Trace the input start
    let preview = remaining.chars().take(40).collect::<String>();
    println!("Parsing file starting with: {}", preview);
    
    // Split the input into chunks for better debugging
    let input_lines: Vec<&str> = input.lines().collect();
    println!("Input has {} lines", input_lines.len());
    for (i, line) in input_lines.iter().take(5).enumerate() {
        println!("Line {}: {}", i, line);
    }
    
    // Parse using directives first
    let mut usings = Vec::new();
    
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
    
    // Now parse top-level members (namespaces, classes)
    let mut members = Vec::new();
    
    // Skip any whitespace between using directives and top-level members
    let (mut remaining, _) = multispace0(remaining)?;
    
    // If there's still input, try to parse top-level members
    while !remaining.trim().is_empty() {
        // Try to parse a top-level member
        match parse_top_level_member(remaining) {
            Ok((rest, member)) => {
                println!("Successfully parsed top-level member");
                members.push(member);
                remaining = rest;
                
                // Skip any whitespace between members
                let (after_ws, _) = multispace0(remaining)?;
                remaining = after_ws;
            },
            Err(e) => {
                println!("Failed to parse top-level member: {:?}", e);
                break;
            }
        }
    }
    
    // Create the source file
    let compilation_unit = CompilationUnit { usings, members };
    
    // Log some debug info
    println!("Parsed source file with {} usings and {} members", 
        compilation_unit.usings.len(), compilation_unit.members.len());
    
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
    
    // Check for 'using' keyword
    if !input.starts_with("using") {
        println!("Not a using directive: doesn't start with 'using'");
        return Err(nom::Err::Error(crate::parser::errors::BSharpParseError::new(
            input, 
            crate::parser::errors::CustomErrorKind::Expected("using")
        )));
    }
    
    // Parse the 'using' keyword
    let (input, _) = keyword("using")(input)?;
    println!("Found 'using' keyword");
    
    // Parse whitespace after 'using'
    let (input, _) = multispace1(input)?;
    
    // Parse the namespace (qualified name)
    let (input, namespace) = nom_to_bs(parse_qualified_name)(input)?;
    
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
fn parse_top_level_member<'a>(input: &'a str) -> BResult<&'a str, TopLevelMember<'a>> {
    println!("Attempting to parse top-level member from: {}", input.chars().take(40).collect::<String>());
    
    // Skip whitespace before trying to parse a top-level member
    let (input, _) = multispace0(input)?;
    
    // We prioritize namespace parsing over class parsing
    alt((
        map(parse_namespace_declaration, TopLevelMember::Namespace),
        map(parse_class_declaration, TopLevelMember::Class),
        map(parse_struct_declaration, TopLevelMember::Struct),
        map(parse_record_declaration, TopLevelMember::Record),
        map(parse_interface_declaration, TopLevelMember::Interface),
        map(parse_enum_declaration, TopLevelMember::Enum),
    ))(input)
}
