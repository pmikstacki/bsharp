// Parser for switch statements

use crate::parser::expressions::pattern_parser::parse_pattern;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::selection_and_switch_keywords::{
    kw_case, kw_default, kw_switch, kw_when,
};
use crate::parser::statement_parser::parse_statement;
use crate::syntax::errors::BResult;
// Need this for statements within sections

use crate::syntax::comment_parser::ws;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::{cut, not, peek};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::expressions::Pattern::Constant;
use syntax::statements::statement::Statement;
use syntax::statements::{SwitchLabel, SwitchSection, SwitchStatement};

// Helper syntax for case labels: case expression:
fn parse_case_label(input: Span) -> BResult<SwitchLabel> {
    (|input| {
            let (input, _) = kw_case().context("case keyword").parse(input)?;

            // Try pattern first
            if let Ok((after_pat, pat)) = delimited(ws, parse_pattern, ws).parse(input) {
                // Optional when clause
                let (after_when, when_clause) = match nom::combinator::opt(preceded(
                    delimited(ws, kw_when(), ws),
                    delimited(ws, parse_expression, ws),
                ))
                .parse(after_pat)
                {
                    Ok((r, w)) => (r, w),
                    Err(_) => (after_pat, None),
                };
                let (after_colon, _) = delimited(ws, nom_char(':'), ws)
                    .context("colon after case")
                    .parse(after_when)?;

                // If it's a simple constant pattern with no when, keep legacy Case(Expression)
                if when_clause.is_none() {
                    if let Constant(expr) = pat {
                        return Ok((after_colon, SwitchLabel::Case(expr)));
                    }
                }

                return Ok((
                    after_colon,
                    SwitchLabel::Pattern {
                        pattern: pat,
                        when_clause,
                    },
                ));
            }

            // Fallback: parse as expression case
            let (input, expr) = delimited(ws, parse_expression, ws)
                .context("case value expression")
                .parse(input)?;
            let (input, _) = delimited(ws, nom_char(':'), ws)
                .context("colon after case value")
                .parse(input)?;
            Ok((input, SwitchLabel::Case(expr)))
        })
    .context("case label")
    .parse(input)
}

// Helper syntax for default label: default:
fn parse_default_label(input: Span) -> BResult<SwitchLabel> {
    map(
        terminated(
            kw_default().context("default keyword"),
            delimited(ws, nom_char(':'), ws).context("colon after default"),
        ),
        |_| SwitchLabel::Default,
    )
    .context("default label")
    .parse(input)
}

// Helper syntax for switch sections (one or more labels followed by zero or more statements)
fn parse_switch_section(input: Span) -> BResult<SwitchSection> {
    (|i| {
        map(
            tuple((
                (|j| {
                    many1(delimited(
                        ws,
                        |x| {
                            if let Ok(r) = parse_case_label(x) { return Ok(r); }
                            parse_default_label(x)
                        },
                        ws,
                    ))
                    .parse(j)
                }),
                (|j| {
                    many0(|k| {
                        // Do not consume statements if the next token starts a new section or closes the switch
                        // Guard against 'case', 'default', or '}'
                        let guard = alt((
                            nom::combinator::map(delimited(ws, kw_case(), ws), |_| ()),
                            nom::combinator::map(delimited(ws, kw_default(), ws), |_| ()),
                            nom::combinator::map(delimited(ws, nom_char('}'), ws), |_| ()),
                        ));
                        peek(not(guard)).parse(k)?;
                        delimited(ws, parse_statement, ws).parse(k)
                    })
                    .parse(j)
                }),
            )),
            |(labels, statements)| SwitchSection { labels, statements },
        )
        .parse(i)
    })
    .context("switch section (expected one or more case/default labels followed by statements)")
    .parse(input)
}

// Original parse_switch_statement function from statement_parser.rs
pub fn parse_switch_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            kw_switch().context("switch keyword"),
            delimited(
                delimited(ws, nom_char('('), ws)
                    .context("opening parenthesis for switch expression"),
                delimited(ws, parse_expression, ws)
                    .context("switch expression"),
                cut(delimited(ws, nom_char(')'), ws))
                    .context("closing parenthesis for switch expression"),
            ),
            delimited(
                delimited(ws, nom_char('{'), ws)
                    .context("opening brace for switch body"),
                |i| many0(|j| delimited(ws, parse_switch_section, ws).parse(j)).parse(i),
                cut(delimited(ws, nom_char('}'), ws))
                    .context("closing brace for switch body"),
            )
            .context("switch body"),
        )),
        |(_, switch_expr, sections)| {
            Statement::Switch(Box::new(SwitchStatement {
                expression: *Box::new(switch_expr),
                sections,
            }))
        },
    )
    .context("switch statement")
    .parse(input)
}
use crate::syntax::span::Span;
