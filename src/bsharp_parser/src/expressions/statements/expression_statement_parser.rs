use crate::syntax::nodes::statements::statement::Statement;
// Parser for expression statements (e.g., x = 5;, DoSomething();)

use crate::parser::expressions::assignment_expression_parser::parse_assignment_expression_or_higher;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{bchar, bws, context};
use nom::combinator::cut;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};

// Parse an expression statement: expression;
pub fn parse_expression_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "expression statement (expected valid C# expression followed by semicolon)",
        |input| {
            // Consume leading whitespace/comments only
            let (input, _) = ws(input)?;
            // Parse the expression core WITHOUT consuming trailing whitespace, to anchor ';' at end of expr
            let (after_expr, expr) = context(
                "expression (expected valid C# expression like assignment, method call, etc.)",
                parse_assignment_expression_or_higher,
            )(input)?;

            // Now require a semicolon. If it's missing, report the error at the end of the expression
            match cut(bws(bchar(';')))(after_expr) {
                Ok((rest, _)) => Ok((rest, Statement::Expression(expr))),
                Err(_) => {
                    // Compute error location anchored at the last non-whitespace character of the consumed span
                    let consumed_len = input.len() - after_expr.len();
                    let prefix = &input[..consumed_len];
                    let trimmed_len = prefix
                        .trim_end_matches(|c: char| c == ' ' || c == '\t' || c == '\r' || c == '\n')
                        .len();
                    let expr_end_loc = &input[trimmed_len..];
                    Err(nom::Err::Failure(ErrorTree::Base {
                        location: expr_end_loc,
                        kind: BaseErrorKind::Expected(Expectation::Char(';')),
                    }))
                }
            }
        },
    )(input)
}
