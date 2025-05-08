use crate::parser::nodes::statements::statement::Statement;
/* Fix the stray closing brace */
// Parser for try-catch-finally statements

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::{CatchClause, FinallyClause, TryStatement};
use crate::parser::parser_helpers::{bchar, bs_context, keyword, nom_to_bs};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::statement_parser::parse_statement_ws;
use crate::parsers::types::type_parser::parse_type_expression;

use nom::{
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
};

// Helper parser for catch clauses
pub fn parse_catch_clause(input: &str) -> BResult<&str, CatchClause> {
    bs_context(
        "catch clause",
        map(
            tuple((
                keyword("catch"),
                multispace0,
                opt(delimited(
                    bchar('('),
                    pair(nom_to_bs(parse_type_expression), opt(preceded(multispace1, nom_to_bs(parse_identifier)))),
                    bchar(')')
                )),
                multispace0,
                parse_statement_ws,
            )),
            |(_, _, opt_type_ident, _, block_stmt)| {
                let (exception_type, exception_variable) = match opt_type_ident {
                    Some((ty, ident_opt)) => (Some(ty), ident_opt),
                    None => (None, None),
                };
                CatchClause {
                    exception_type,
                    exception_variable,
                    block: Box::new(block_stmt),
                }
            }
        )
    )(input)
}

// Helper parser for the finally clause
pub fn parse_finally_clause(input: &str) -> BResult<&str, FinallyClause> {
    bs_context(
        "finally clause",
        map(
            preceded(
                keyword("finally"),
                preceded(multispace0, parse_statement_ws),
            ),
            |block_stmt| FinallyClause { block: Box::new(block_stmt) },
        ),
    )(input)
}

// Parse a try-catch-finally statement
pub fn parse_try_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "try statement",
        map(
            tuple((
                keyword("try"),
                multispace0,
                parse_statement_ws,
                many0(parse_catch_clause),
                opt(parse_finally_clause),
            )),
            |(_, _, try_block_stmt, catch_clauses, finally_clause)| {
                Statement::Try(Box::new(TryStatement {
                    try_block: Box::new(try_block_stmt),
                    catches: catch_clauses,
                    finally_clause,
                }))
            },
        ),
    )(input)
}
