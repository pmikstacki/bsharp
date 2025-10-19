// Parser for if/else statements

use nom::combinator::cut;
use nom::combinator::opt;
use nom::sequence::{delimited, preceded};
use nom::Parser;

use crate::parser::keywords::selection_and_switch_keywords::{kw_else, kw_if};
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement_ws;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::IfStatement;

/// Parse an if statement with optional else branch
/// Format: if (expr) stmt [else stmt]
/// Note: In C#, if statements MUST have block bodies (braces are required)
pub fn parse_if_statement(input: Span) -> BResult<Statement> {
    (|input| {
        // if keyword
        let (input, _) = kw_if().context("if keyword").parse(input)?;
        // (
        let (input, _) = delimited(ws, tok_l_paren(), ws)
            .context("opening parenthesis for if condition")
            .parse(input)?;
        // condition expr
        let (input, condition) = delimited(ws, parse_expression, ws).parse(input)?;
        // )
        let (after_paren, _) = cut(delimited(ws, tok_r_paren(), ws))
            .context("closing parenthesis for if condition")
            .parse(input)?;

        // then statement, committed; on failure, report at after_paren location
        let (input, then_branch) = match cut(delimited(ws, parse_statement_ws, ws)).parse(after_paren)
        {
            Ok(ok) => ok,
            Err(_) => {
                return Err(nom::Err::Failure(ErrorTree::Base {
                    location: after_paren,
                    kind: BaseErrorKind::Expected(Expectation::Tag("statement")),
                }));
            }
        };

        // optional else
        let (input, else_branch) = opt(preceded(
            delimited(ws, kw_else(), ws),
            |i| match cut(delimited(ws, parse_statement_ws, ws)).parse(i) {
                Ok(ok) => Ok(ok),
                Err(_) => Err(nom::Err::Failure(ErrorTree::Base {
                    location: i,
                    kind: BaseErrorKind::Expected(Expectation::Tag("statement")),
                })),
            },
        ))
            .parse(input)?;

        Ok((
            input,
            Statement::If(Box::new(IfStatement {
                condition,
                consequence: Box::new(then_branch),
                alternative: else_branch.map(Box::new),
            })),
        ))
    })
        .context("if statement")
        .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
