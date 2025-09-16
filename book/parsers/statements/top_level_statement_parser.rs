use crate::parser::errors::BResult;
use crate::parser::nodes::statements::statement::Statement;
use crate::parsers::statement_parser::parse_statement;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::sequence::terminated;

/// Parse top-level statements (C# 9+)
/// Top-level statements are statements that appear at the top level of a file,
/// outside of any class or method. They are implicitly wrapped in a Main method.
/// 
/// Example:
/// ```csharp
/// using System;
/// 
/// Console.WriteLine("Hello, World!");
/// var name = "Alice";
/// Console.WriteLine($"Hello, {name}!");
/// ```
pub fn parse_top_level_statements(input: &str) -> BResult<&str, Vec<Statement>> {
    println!("[DEBUG] parse_top_level_statements: input = {:?}", input.chars().take(60).collect::<String>());
    
    // Parse 0 or more statements at the top level
    let (input, statements) = many0(
        terminated(
            parse_statement,
            multispace0 // consume whitespace after each statement
        )
    )(input)?;
    
    println!("[DEBUG] parse_top_level_statements: parsed {} statements", statements.len());
    
    Ok((input, statements))
}

/// Parse a single top-level statement
/// This is a wrapper around parse_statement that handles top-level context
pub fn parse_top_level_statement(input: &str) -> BResult<&str, Statement> {
    println!("[DEBUG] parse_top_level_statement: input = {:?}", input.chars().take(60).collect::<String>());
    
    // Top-level statements can be any regular statement
    parse_statement(input)
} 