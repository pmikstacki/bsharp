use crate::syntax::nodes::statements::statement::Statement;
// Parser for switch statements

use crate::parser::expressions::pattern_parser::parse_pattern;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::selection_and_switch_keywords::{
    kw_case, kw_default, kw_switch, kw_when,
};
use crate::parser::statement_parser::parse_statement;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, bws, context};
// Need this for statements within sections

use crate::syntax::comment_parser::ws;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::{cut, not, peek};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};

// Helper syntax for case labels: case expression:
fn parse_case_label(input: &str) -> BResult<&str, SwitchLabel> {
    context(
        "case label (expected 'case <pattern|expression>[: when <expr>]:')",
        |input| {
            let (input, _) = context("case keyword (expected 'case')", kw_case())(input)?;

            // Try pattern first
            if let Ok((after_pat, pat)) = bws(parse_pattern)(input) {
                // Optional when clause
                let (after_when, when_clause) =
                    match nom::combinator::opt(preceded(bws(kw_when()), bws(parse_expression)))(
                        after_pat,
                    ) {
                        Ok((r, w)) => (r, w),
                        Err(_) => (after_pat, None),
                    };
                let (after_colon, _) =
                    context("colon after case (expected ':')", bws(bchar(':')))(after_when)?;

                // If it's a simple constant pattern with no when, keep legacy Case(Expression)
                if when_clause.is_none() {
                    if let crate::syntax::nodes::expressions::pattern::Pattern::Constant(expr) = pat
                    {
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
            let (input, expr) = context(
                "case value expression (expected valid C# expression)",
                bws(parse_expression),
            )(input)?;
            let (input, _) =
                context("colon after case value (expected ':')", bws(bchar(':')))(input)?;
            Ok((input, SwitchLabel::Case(expr)))
        },
    )(input)
}

// Helper syntax for default label: default:
fn parse_default_label(input: &str) -> BResult<&str, SwitchLabel> {
    context(
        "default label (expected 'default:')",
        map(
            terminated(
                context("default keyword (expected 'default')", kw_default()),
                context("colon after default (expected ':')", bws(bchar(':'))),
            ),
            |_| SwitchLabel::Default,
        ),
    )(input)
}

// Helper syntax for switch sections (one or more labels followed by zero or more statements)
fn parse_switch_section(input: &str) -> BResult<&str, SwitchSection> {
    context(
        "switch section (expected one or more case/default labels followed by statements)",
        map(
            tuple((
                context(
                    "switch section labels (expected one or more 'case' or 'default' labels)",
                    many1(delimited(
                        ws,
                        context(
                            "switch label (expected 'case expression:' or 'default:')",
                            alt((parse_case_label, parse_default_label)),
                        ),
                        ws,
                    )),
                ),
                context(
                    "switch section statements (expected zero or more C# statements)",
                    many0(|i| {
                        // Do not consume statements if the next token starts a new section or closes the switch
                        // Guard against 'case', 'default', or '}'
                        let mut guard = alt((
                            nom::combinator::map(bws(kw_case()), |_| ()),
                            nom::combinator::map(bws(kw_default()), |_| ()),
                            nom::combinator::map(bws(bchar('}')), |_| ()),
                        ));
                        peek(not(&mut guard))(i)?;
                        delimited(ws, parse_statement, ws)(i)
                    }),
                ),
            )),
            |(labels, statements)| SwitchSection { labels, statements },
        ),
    )(input)
}

// Original parse_switch_statement function from statement_parser.rs
pub fn parse_switch_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "switch statement (expected 'switch (expression) { case/default sections }')",
        map(
            tuple((
                context(
                    "whitespace after switch keyword",
                    preceded(
                        context("switch keyword (expected 'switch')", kw_switch()),
                        ws,
                    ),
                ),
                context(
                    "switch expression in parentheses (expected '(expression)')",
                    delimited(
                        context(
                            "opening parenthesis for switch expression (expected '(')",
                            bchar('('),
                        ),
                        context(
                            "switch expression (expected valid C# expression)",
                            delimited(ws, parse_expression, ws),
                        ),
                        context(
                            "closing parenthesis for switch expression (expected ')')",
                            cut(bchar(')')),
                        ),
                    ),
                ),
                context("whitespace after switch expression", ws),
                context(
                    "switch body (expected '{' followed by case/default sections and '}')",
                    delimited(
                        context("opening brace for switch body (expected '{')", bchar('{')),
                        context(
                            "switch sections (expected zero or more case/default sections)",
                            many0(delimited(ws, parse_switch_section, ws)),
                        ),
                        context(
                            "closing brace for switch body (expected '}')",
                            cut(bws(bchar('}'))),
                        ),
                    ),
                ),
            )),
            |(_, switch_expr, _, sections)| {
                Statement::Switch(Box::new(SwitchStatement {
                    expression: *Box::new(switch_expr),
                    sections,
                }))
            },
        ),
    )(input)
}
