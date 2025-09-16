use crate::syntax::nodes::statements::statement::Statement;
// Parser for try-catch-finally statements

use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::{CatchClause, FinallyClause, TryStatement};
use crate::syntax::parser_helpers::{bchar, context, bws, keyword};
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::statement_parser::parse_statement_ws;
use crate::parser::types::type_parser::parse_type_expression;

use nom::{
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, pair, tuple},
};
use nom::combinator::cut;
use crate::syntax::comment_parser::ws;

// Helper syntax for catch clauses, following Roslyn's structure
pub fn parse_catch_clause(input: &str) -> BResult<&str, CatchClause> {
    context(
        "catch clause (expected 'catch' with optional (type variable) and block)",
        map(
            tuple((
                context("catch keyword (expected 'catch')", keyword("catch")),
                context("whitespace after catch keyword", ws),
                // 3. Optional exception type and variable in parentheses
                context("optional catch type/variable (expected '(Type variable)') or nothing", opt(delimited(
                    context("opening parenthesis for catch (expected '(')", bws(bchar('('))),
                    pair(
                        context("exception type in catch (expected valid C# type)", bws(parse_type_expression)),
                        context("optional exception variable in catch (expected identifier or nothing)", opt(bws(parse_identifier)))
                    ),
                    cut(context("closing parenthesis for catch (expected ')')", bws(bchar(')'))))
                ))),
                context("whitespace before catch block", ws),
                context("catch block (expected valid C# statement block)", bws(parse_statement_ws)),
            )),
            |(_catch_kw, _, exception_info, _, block_stmt)| {
                // Extract exception type and variable if provided
                let (exception_type, exception_variable) = match exception_info {
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

// Helper syntax for the finally clause, following Roslyn's structure
pub fn parse_finally_clause(input: &str) -> BResult<&str, FinallyClause> {
    context(
        "finally clause (expected 'finally' and block)",
        map(
            tuple((
                context("finally keyword (expected 'finally')", keyword("finally")),
                context("whitespace after finally keyword", ws),
                context("finally block (expected valid C# statement block)", bws(parse_statement_ws)),
            )),
            |(_finally_kw, _, block_stmt)| {
                FinallyClause { 
                    block: Box::new(block_stmt) 
                }
            },
        ),
    )(input)
}

// Parse a try-catch-finally statement, following Roslyn's structure
pub fn parse_try_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "try statement (expected 'try' block, zero or more 'catch' clauses, and optional 'finally' block)",
        map(
            tuple((
                context("try keyword (expected 'try')", keyword("try")),
                context("try block (expected valid C# statement block)", bws(parse_statement_ws)),
                context("zero or more catch clauses", many0(bws(parse_catch_clause))),
                context("optional finally clause", opt(bws(parse_finally_clause)))
            )),
            |(_, try_block, catch_clauses, finally_clause)| {
                Statement::Try(Box::new(TryStatement {
                    try_block: Box::new(try_block),
                    catches: catch_clauses,
                    finally_clause,
                }))
            }
        )
    )(input)
}
