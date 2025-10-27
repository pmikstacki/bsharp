// Parser for while statements

use crate::parser::keywords::iteration_keywords::kw_while;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::combinator::cut;
use nom::{combinator::map, sequence::delimited};
use nom_supreme::ParserExt;
use syntax::statements::WhileStatement;
use syntax::statements::statement::Statement;

// Parse a while statement
pub fn parse_while_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_while().context("while keyword"),
            delimited(
                delimited(ws, tok_l_paren(), ws),
                (|i| crate::parser::expressions::primary_expression_parser::parse_expression(i))
                    .context("while loop condition"),
                cut(delimited(ws, tok_r_paren(), ws))
                    .context("closing parenthesis for while condition"),
            ),
            cut(delimited(
                ws,
                |i| crate::parser::statement_parser::parse_statement_ws(i),
                ws,
            ))
            .context("while loop body"),
        ),
        |(_, condition, body_statement)| {
            Statement::While(Box::new(WhileStatement {
                condition: Box::new(condition),
                body: Box::new(body_statement),
            }))
        },
    )
    .context("while statement")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
