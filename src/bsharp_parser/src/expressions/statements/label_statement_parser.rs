use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::label_statement::LabelStatement;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, bws};

use nom::Parser;
use nom::{combinator::map, sequence::tuple};
use nom_supreme::ParserExt;

/// Parse a label statement: identifier:
pub fn parse_label_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            parse_identifier.context("label identifier"),
            bws(bchar(':')).context("colon after label"),
        )),
        |(label, _)| Statement::Label(LabelStatement { label }),
    )
    .context("label statement")
    .parse(input)
}
