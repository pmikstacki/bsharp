// Parser for break statements
use crate::parser::keywords::flow_control_keywords::kw_break;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::cut;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::BreakStatement;

// Original parse_break_statement function from statement_parser.rs
pub fn parse_break_statement(input: Span) -> BResult<Statement> {
    map(
        terminated(
            kw_break().context("break keyword"),
            cut(delimited(ws, tok_semicolon(), ws))
                .context("semicolon after break statement"),
        ),
        |_| Statement::Break(BreakStatement),
    )
        .context("break statement")
        .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::separators::tok_semicolon;
