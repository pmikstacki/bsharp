use crate::syntax::nodes::statements::statement::Statement;
// Parser for if/else statements

use nom::combinator::cut;
use nom::combinator::opt;
use nom::sequence::preceded;

use crate::parser::keywords::selection_and_switch_keywords::{kw_else, kw_if};
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, bws, context, label};

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement_ws;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};

/// Parse an if statement with optional else branch
/// Format: if (expr) stmt [else stmt]
/// Note: In C#, if statements MUST have block bodies (braces are required)
pub fn parse_if_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "if statement (expected 'if (condition) statement' with optional 'else statement')",
        |input| {
            // if keyword
            let (input, _) = context("if keyword (expected 'if')", kw_if())(input)?;
            // (
            let (input, _) = context(
                "opening parenthesis for if condition (expected '(')",
                bws(bchar('(')),
            )(input)?;
            // condition expr
            let (input, condition) = bws(parse_expression)(input)?;
            // )
            let (after_paren, _) = cut(context(
                "closing parenthesis for if condition (expected ')')",
                bchar(')'),
            ))(input)?;

            // then statement, committed; on failure, report at after_paren location
            let (input, then_branch) =
                match cut(label("after_paren", bws(parse_statement_ws)))(after_paren) {
                    Ok(ok) => ok,
                    Err(_) => {
                        return Err(nom::Err::Failure(ErrorTree::Base {
                            location: after_paren,
                            kind: BaseErrorKind::Expected(Expectation::Tag("statement")),
                        }));
                    }
                };

            // optional else
            let (input, else_branch) = opt(preceded(bws(kw_else()), |i| {
                match cut(label("after_else", bws(parse_statement_ws)))(i) {
                    Ok(ok) => Ok(ok),
                    Err(_) => Err(nom::Err::Failure(ErrorTree::Base {
                        location: i,
                        kind: BaseErrorKind::Expected(Expectation::Tag("statement")),
                    })),
                }
            }))(input)?;

            Ok((
                input,
                Statement::If(Box::new(IfStatement {
                    condition,
                    consequence: Box::new(then_branch),
                    alternative: else_branch.map(Box::new),
                })),
            ))
        },
    )(input)
}
