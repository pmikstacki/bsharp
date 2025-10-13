// Parser for expression statements (e.g., x = 5;, DoSomething();)

use crate::parser::expressions::assignment_expression_parser::parse_assignment_expression_or_higher;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::cut;
use nom::character::complete::char as nom_char;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;

// Parse an expression statement: expression;
pub fn parse_expression_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    (|input| {
        let (input, _) = ws(input)?;
        let (after_expr, expr) = parse_assignment_expression_or_higher
            .context("expression")
            .parse(input)?;
        let (rest, _) = cut(delimited(ws, nom_char(';'), ws))
            .context("semicolon after expression")
            .parse(after_expr)?;
        Ok((rest, Statement::Expression(expr)))
    })
    .context("expression statement")
    .parse(input)
}
use crate::syntax::span::Span;
