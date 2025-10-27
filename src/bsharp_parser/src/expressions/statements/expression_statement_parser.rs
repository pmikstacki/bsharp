// Parser for expression statements (e.g., x = 5;, DoSomething();)

use crate::parser::expressions::assignment_expression_parser::parse_assignment_expression_or_higher;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::combinator::cut;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;

// Parse an expression statement: expression;
pub fn parse_expression_statement(input: Span) -> BResult<Statement> {
    (|input| {
        let (input, _) = ws(input)?;
        let (after_expr, expr) = parse_assignment_expression_or_higher
            .context("expression")
            .parse(input)?;
        let (rest, _) = cut(delimited(ws, tok_semicolon(), ws))
            .context("semicolon after expression")
            .parse(after_expr)?;
        Ok((rest, Statement::Expression(expr)))
    })
    .context("expression statement")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::separators::tok_semicolon;
