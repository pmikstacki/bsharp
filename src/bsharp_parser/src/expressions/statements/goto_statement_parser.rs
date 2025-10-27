use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::flow_control_keywords::kw_goto;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::combinator::cut;
use nom::combinator::map;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use syntax::statements::GotoStatement;
use syntax::statements::statement::Statement;

/// Parse a goto statement: goto label;
pub fn parse_goto_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_goto().context("goto keyword"),
            delimited(ws, parse_identifier, ws).context("label identifier"),
            cut(delimited(ws, tok_semicolon(), ws)).context("semicolon after goto statement"),
        ),
        |(_, label, _)| Statement::Goto(GotoStatement { label }),
    )
    .context("goto statement")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::separators::tok_semicolon;
