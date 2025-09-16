use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::errors::BResult;
use crate::parser::statement_parser::parse_statement_for_block_ws;
use crate::syntax::comment_parser::parse_whitespace_or_comments;
use nom::character::complete::char;
use nom::sequence::{delimited, preceded};
use nom::multi::many0;
use nom::combinator::map;
use nom::error::context;

// Function to extract statements from a Block statement, if that's what it is
// This is a helper for tests and other code that expects a Vec<Statement>
pub fn extract_statements_from_block(statement: Statement) -> Vec<Statement> {
    match statement {
        Statement::Block(statements) => statements,
        _ => vec![statement], // If it's not a block, wrap it in a Vec as a fallback
    }
}

/// Block statement syntax with enhanced error context
/// Uses nom's context function for better error messages
/// Uses parse_statement_for_block_ws to prevent infinite recursion with nested blocks
pub fn parse_block_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "block statement (expected '{' followed by zero or more statements and '}')",
        map(
            delimited(
                context("opening brace for block statement (expected '{')", char('{')),
                many0(context("statement in block (expected valid C# statement)", parse_statement_for_block_ws)),
                context("closing brace for block statement (expected '}')", preceded(parse_whitespace_or_comments, char('}')))
            ),
            |statements| Statement::Block(statements)
        )
    )(input)
} 