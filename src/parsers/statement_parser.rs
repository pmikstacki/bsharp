use crate::parser::nodes::statements::statement::Statement;
use crate::parser::errors::BResult;
use crate::parsers::declarations::variable_declaration_parser::parse_local_variable_declaration;
use crate::parsers::statements::*;

use nom::branch::alt;
use nom::combinator::map;
use nom::character::complete::multispace0;
use nom::sequence::delimited;

/// Main statement parser - handles all types of statements
/// This function correctly handles recursion by dispatching to specific statement parsers
pub fn parse_statement<'a>(input: &'a str) -> BResult<&'a str, Statement<'a>> {
    // First handle any leading whitespace to improve parsing success rate
    let (input, _) = multispace0(input)?;
    
    // Order matters here - more specific patterns should come first
    alt((
        // Jump statements - these need to come first to avoid variable declarations
        // matching keywords like "return" as types
        parse_return_statement,
        parse_break_statement,
        parse_continue_statement,
        parse_throw_statement,
        
        // Control flow statements
        parse_if_statement,
        parse_for_statement,
        parse_foreach_statement,
        parse_while_statement,
        parse_do_while_statement,
        parse_switch_statement,
        
        // Exception handling
        parse_try_statement,
        
        // Block statements
        parse_block_statement,
        
        // Declaration statements 
        map(parse_local_variable_declaration, Statement::Declaration),
        
        // Expression statements come last (most general case)
        parse_expression_statement,
    ))(input)
}

/// Parse a statement and handle trailing whitespace
/// This helper function is useful for statement parsers that need to ensure whitespace is handled
pub fn parse_statement_ws<'a>(input: &'a str) -> BResult<&'a str, Statement<'a>> {
    delimited(multispace0, parse_statement, multispace0)(input)
}
