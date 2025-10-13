// Parser for try-catch-finally statements

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::exception_and_safety_keywords::{kw_catch, kw_finally, kw_try};
use crate::parser::keywords::selection_and_switch_keywords::kw_when;
use crate::parser::statement_parser::parse_statement_ws;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;
use nom::combinator::cut;
use nom::sequence::preceded;
use nom::{
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, pair, tuple},
};
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::{CatchClause, FinallyClause, TryStatement};

// Helper syntax for catch clauses, following Roslyn's structure
pub fn parse_catch_clause<'a>(input: Span<'a>) -> BResult<'a, CatchClause> {
    map(
        tuple((
            kw_catch().context("catch keyword"),
            // Optional exception (Type ident)
            opt(
                delimited(
                    delimited(ws, nom_char('('), ws)
                        .context("opening parenthesis for catch"),
                    pair(
                        delimited(ws, parse_type_expression, ws)
                            .context("exception type in catch"),
                        opt(delimited(ws, parse_identifier, ws)).context(
                            "optional exception variable in catch",
                        ),
                    ),
                    cut(delimited(ws, nom_char(')'), ws))
                        .context("closing parenthesis for catch"),
                ),
            )
            .context("optional catch type/variable"),
            // Optional when filter: when (expr)
            opt(preceded(
                delimited(ws, kw_when(), ws),
                delimited(
                    delimited(ws, nom_char('('), ws),
                    delimited(ws, parse_expression, ws),
                    cut(delimited(ws, nom_char(')'), ws)),
                ),
            )),
            cut(delimited(ws, parse_statement_ws, ws))
                .context("catch block"),
        )),
        |(_catch_kw, exception_info, when_clause, block_stmt)| {
            let (exception_type, exception_variable) = match exception_info {
                Some((ty, ident_opt)) => (Some(ty), ident_opt),
                None => (None, None),
            };
            CatchClause {
                exception_type,
                exception_variable,
                when_clause,
                block: Box::new(block_stmt),
            }
        },
    )
    .context("catch clause")
    .parse(input)
}

// Helper syntax for the finally clause, following Roslyn's structure
pub fn parse_finally_clause<'a>(input: Span<'a>) -> BResult<'a, FinallyClause> {
    map(
        tuple((
            kw_finally().context("finally keyword"),
            cut(delimited(ws, parse_statement_ws, ws))
                .context("finally block"),
        )),
        |(_finally_kw, block_stmt)| FinallyClause { block: Box::new(block_stmt) },
    )
    .context("finally clause")
    .parse(input)
}

// Parse a try-catch-finally statement, following Roslyn's structure
pub fn parse_try_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            kw_try().context("try keyword"),
            cut(delimited(ws, parse_statement_ws, ws))
                .context("try block"),
            many0(delimited(ws, parse_catch_clause, ws))
                .context("zero or more catch clauses"),
            opt(delimited(ws, parse_finally_clause, ws))
                .context("optional finally clause"),
        )),
        |(_, try_block, catch_clauses, finally_clause)| {
            Statement::Try(Box::new(TryStatement {
                try_block: Box::new(try_block),
                catches: catch_clauses,
                finally_clause,
            }))
        },
    )
    .context("try statement")
    .parse(input)
}
use crate::syntax::span::Span;
