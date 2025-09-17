use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::context;
use crate::syntax::parser_helpers::{bchar, bws};
use nom::combinator::map;
use nom::combinator::{cut, not, peek};
use nom::multi::many0;
use nom::sequence::delimited;

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
                context(
                    "opening brace for block statement (expected '{')",
                    bws(bchar('{')),
                ),
                many0(context(
                    "statement in block (expected valid C# statement)",
                    |i| {
                        // Do not attempt a statement if next non-ws is '}'
                        peek(not(bws(bchar('}'))))(i)?;
                        parse_statement_ws(i)
                    },
                )),
                context(
                    "closing brace for block statement (expected '}')",
                    cut(bws(bchar('}'))),
                ),
            ),
            |statements| Statement::Block(statements),
        ),
    )(input)
}
