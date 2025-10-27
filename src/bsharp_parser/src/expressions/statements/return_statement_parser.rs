// Parser for return statements

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::flow_control_keywords::kw_return;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::combinator::cut;
use nom::{
    combinator::{map, opt},
    sequence::delimited,
};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;

// Parse a return statement with an optional expression
pub fn parse_return_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_return().context("return keyword"),
            // Optional expression preceded by whitespace
            opt(delimited(ws, parse_expression, ws)).context("optional return value"),
            cut(delimited(ws, tok_semicolon(), ws)).context("semicolon after return statement"),
        ),
        |(_, expr_opt, _)| Statement::Return(expr_opt.map(Box::new)),
    )
    .context("return statement")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::separators::tok_semicolon;
