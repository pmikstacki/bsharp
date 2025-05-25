use crate::parser::nodes::statements::statement::Statement;
// Parser for switch statements

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::*;
use crate::parser::parser_helpers::{bchar, bs_context, keyword};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::statement_parser::parse_statement;
// Need this for statements within sections

use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};

// Helper parser for case labels: case expression:
fn parse_case_label(input: &str) -> BResult<&str, SwitchLabel> {
    map(
        preceded(
            keyword("case"),
            terminated(
                preceded(multispace0, parse_expression),
                preceded(multispace0, bchar(':')),
            ),
        ),
        |expr| SwitchLabel::Case(expr),
    )(input)
}

// Helper parser for default label: default:
fn parse_default_label(input: &str) -> BResult<&str, SwitchLabel> {
    map(terminated(keyword("default"), preceded(multispace0, bchar(':'))), |_| {
        SwitchLabel::Default
    })(input)
}

// Helper parser for switch sections (one or more labels followed by zero or more statements)
fn parse_switch_section(input: &str) -> BResult<&str, SwitchSection> {
    map(
        tuple((
            many1(delimited(
                multispace0,
                alt((parse_case_label, parse_default_label)),
                multispace0,
            )),
            many0(delimited(multispace0, parse_statement, multispace0)),
        )),
        |(labels, statements)| SwitchSection { labels, statements },
    )(input)
}

// Original parse_switch_statement function from statement_parser.rs
pub fn parse_switch_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "switch statement",
        map(
            tuple((
                preceded(keyword("switch"), multispace0),
                delimited(bchar('('), delimited(multispace0, parse_expression, multispace0), bchar(')')),
                multispace0,
                delimited(
                    bchar('{'),
                    many0(delimited(multispace0, parse_switch_section, multispace0)),
                    preceded(multispace0, bchar('}')),
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
