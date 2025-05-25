// Improved whitespace handling for declaration parsers
// This module provides common helpers for parsing C# declarations with robust whitespace handling

use nom::{
    character::complete::{multispace0, multispace1},
    bytes::complete::tag as nom_tag,
    IResult,
    error::Error as NomError,
};
use crate::parser::errors::BResult;
use crate::parser::parser_helpers::nom_to_bs;
use crate::parser::nodes::declarations::Modifier;

/// Robustly parses a keyword with optional surrounding whitespace
pub fn parse_keyword<'a>(keyword: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, NomError<&'a str>> {
    move |input: &'a str| {
        // First consume any whitespace before the keyword
        let (input, _) = multispace0::<&str, NomError<&str>>(input)?;
        
        // Parse the actual keyword
        let tag_parser = nom_tag::<&'static str, &'a str, NomError<&'a str>>(keyword);
        let (input, kw) = tag_parser(input)?;
        
        // Ensure there's at least some whitespace after the keyword
        let (input, _) = multispace1::<&str, NomError<&str>>(input)?;
        
        Ok((input, kw))
    }
}

/// Helper for consuming optional whitespace
pub fn optional_whitespace(input: &str) -> IResult<&str, &str, NomError<&str>> {
    multispace0(input)
}

/// Helper for robustly handling modifiers followed by a keyword
pub fn parse_declaration_header<'a, F>(
    mut modifiers_parser: F,
    keyword: &'static str
) -> impl FnMut(&'a str) -> BResult<&'a str, (Vec<Modifier>, &'a str)>
where
    F: FnMut(&'a str) -> BResult<&'a str, Vec<Modifier>>
{
    move |input: &'a str| {
        // Parse modifiers (which might be empty)
        let (input, modifiers) = modifiers_parser(input)?;
        
        // Parse the keyword (struct, interface, etc.)
        let (input, kw) = nom_to_bs(parse_keyword(keyword))(input)?;
        
        Ok((input, (modifiers, kw)))
    }
}
