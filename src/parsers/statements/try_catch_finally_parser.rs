use crate::parser::nodes::statements::statement::Statement;
// Parser for try-catch-finally statements

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::{CatchClause, FinallyClause, TryStatement};
use crate::parser::parser_helpers::{bchar, bs_context, keyword, nom_to_bs, bws};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::statement_parser::parse_statement_ws;
use crate::parsers::types::type_parser::parse_type_expression;

use nom::{
    character::complete::multispace0,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, pair, tuple},
};

// Helper parser for catch clauses, following Roslyn's structure
pub fn parse_catch_clause(input: &str) -> BResult<&str, CatchClause> {
    bs_context(
        "catch clause",
        map(
            tuple((
                // 1. Catch keyword
                keyword("catch"),
                // 2. Handle whitespace after catch keyword
                multispace0,
                // 3. Optional exception type and variable in parentheses
                opt(delimited(
                    bws(bchar('(')),
                    pair(
                        // Exception type
                        bws(nom_to_bs(parse_type_expression)), 
                        // Optional exception variable name
                        opt(bws(nom_to_bs(parse_identifier)))
                    ),
                    bws(bchar(')'))
                )),
                // 4. Handle any whitespace or comments before the block statement
                multispace0, 
                // 5. Block statement (usually enclosed in { })
                bws(parse_statement_ws),
            )),
            |(_catch_kw, _, exception_info, _, block_stmt)| {
                // Extract exception type and variable if provided
                let (exception_type, exception_variable) = match exception_info {
                    Some((ty, ident_opt)) => (Some(ty), ident_opt),
                    None => (None, None),
                };
                
                // Create the catch clause node
                CatchClause {
                    exception_type,
                    exception_variable,
                    block: Box::new(block_stmt),
                }
            }
        )
    )(input)
}

// Helper parser for the finally clause, following Roslyn's structure
pub fn parse_finally_clause(input: &str) -> BResult<&str, FinallyClause> {
    bs_context(
        "finally clause",
        map(
            tuple((
                // 1. Finally keyword
                keyword("finally"),
                // 2. Handle whitespace after finally keyword
                multispace0,
                // 3. Block statement (usually enclosed in { })
                bws(parse_statement_ws),
            )),
            |(_finally_kw, _, block_stmt)| {
                // Create the finally clause node
                FinallyClause { 
                    block: Box::new(block_stmt) 
                }
            },
        ),
    )(input)
}

// Parse a try-catch-finally statement, following Roslyn's structure
pub fn parse_try_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "try statement",
        map(
            tuple((
                // 1. Try keyword
                keyword("try"),
                // 2. Try block statement
                bws(parse_statement_ws),
                // 3. Zero or more catch clauses
                many0(bws(parse_catch_clause)),
                // 4. Optional finally clause
                opt(bws(parse_finally_clause))
            )),
            |(_, try_block, catch_clauses, finally_clause)| {
                // Create the try statement node
                Statement::Try(Box::new(TryStatement {
                    try_block: Box::new(try_block),
                    catches: catch_clauses,
                    finally_clause,
                }))
            }
        )
    )(input)
}
