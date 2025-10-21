use nom::multi::many0;
use nom::{branch::alt, combinator::map};

use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::type_declaration_parser::convert_attributes;
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::keywords::accessor_keywords::{kw_get, kw_set};
use crate::parser::keywords::contextual_misc_keywords::kw_this;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::list_parser::parse_delimited_list0;
use nom::character::complete::satisfy;
use nom::combinator::{cut, peek};
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::declarations::{IndexerAccessor, IndexerAccessorList, IndexerDeclaration};
use syntax::types::Parameter;

/// Parse a C# indexer declaration
///
/// Examples:
/// ```csharp
/// public int this[int index] { get; set; }
/// public string this[int row, int col] { get { return _data[row][col]; } set { _data[row][col] = value; } }
/// ```
pub fn parse_indexer_declaration(input: Span) -> BResult<IndexerDeclaration> {
    // Parse attributes, modifiers, type, 'this', parameters, and accessor list
    // Attributes
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    let attributes = convert_attributes(attribute_lists);
    // Modifiers
    let (input, modifiers) = parse_modifiers(input)?;
    // Return type
    let (input, ty) = delimited(ws, parse_type_expression, ws)
        .context("indexer return type")
        .parse(input)?;
    // 'this'
    let (input, _) = delimited(ws, kw_this(), ws)
        .context("this keyword")
        .parse(input)?;
    // Parameters
    let (input, parameters) = parse_indexer_parameters(input)?;
    // Accessor list
    let (input, accessor_list) = parse_indexer_accessor_list(input)?;

    let indexer_declaration = IndexerDeclaration {
        attributes,
        modifiers,
        indexer_type: ty,
        parameters,
        accessor_list,
    };

    Ok((input, indexer_declaration))
}

/// Parse indexer parameters (same as regular parameters but inside square brackets)
fn parse_indexer_parameters(input: Span) -> BResult<Vec<Parameter>> {
    use crate::parser::expressions::declarations::parameter_parser::parse_parameter;

    parse_delimited_list0::<_, _, _, _, char, char, char, Parameter>(
        |i| delimited(ws, satisfy(|c| c == '['), ws).parse(i),
        |i| delimited(ws, parse_parameter, ws).parse(i),
        |i| delimited(ws, tok_comma(), ws).parse(i),
        |i| delimited(ws, satisfy(|c| c == ']'), ws).parse(i),
        false,
        true,
    )
        .context("indexer parameters")
        .parse(input)
}

/// Parse the indexer accessor list
fn parse_indexer_accessor_list(input: Span) -> BResult<IndexerAccessorList> {
    // Parse opening brace
    let (input, _) = delimited(ws, satisfy(|c| c == '{'), ws)
        .context("accessor list opening brace")
        .parse(input)?;

    // Parse accessors (get and/or set)
    let (input, (get_accessor, set_accessor)) = parse_accessors(input)?;

    // Parse closing brace
    let (input, _) = cut(delimited(ws, satisfy(|c| c == '}'), ws))
        .context("accessor list closing brace")
        .parse(input)?;

    Ok((
        input,
        IndexerAccessorList {
            get_accessor,
            set_accessor,
        },
    ))
}

/// Parse get and/or set accessors
fn parse_accessors(
    input: Span,
) -> BResult<(Option<IndexerAccessor>, Option<IndexerAccessor>)> {
    // Two accessor parsers with lookahead over optional attrs/modifiers
    let get_branch = |i| {
        let (i, _) = peek(|ii| {
            let (ii, _) = delimited(ws, parse_attribute_lists, ws).parse(ii)?;
            let (ii, _) = delimited(ws, parse_modifiers, ws).parse(ii)?;
            let (ii, _) = delimited(ws, kw_get(), ws).parse(ii)?;
            Ok((ii, ()))
        }).parse(i)?;
        parse_get_accessor_declaration(i)
    };
    let set_branch = |i| {
        let (i, _) = peek(|ii| {
            let (ii, _) = delimited(ws, parse_attribute_lists, ws).parse(ii)?;
            let (ii, _) = delimited(ws, parse_modifiers, ws).parse(ii)?;
            let (ii, _) = delimited(ws, kw_set(), ws).parse(ii)?;
            Ok((ii, ()))
        }).parse(i)?;
        parse_set_accessor_declaration(i)
    };

    let one_accessor = |i| {
        alt((
            map(get_branch, |a| (true, a)),
            map(set_branch, |a| (false, a)),
        ))
            .parse(i)
    };

    let (cur, pairs) = many0(one_accessor).parse(input)?;
    let mut get_accessor: Option<IndexerAccessor> = None;
    let mut set_accessor: Option<IndexerAccessor> = None;
    for (is_get, accessor) in pairs {
        if is_get {
            get_accessor = Some(accessor);
        } else {
            set_accessor = Some(accessor);
        }
    }

    Ok((cur, (get_accessor, set_accessor)))
}

/// Parse a get accessor declaration
fn parse_get_accessor_declaration(input: Span) -> BResult<IndexerAccessor> {
    // Optional attribute lists and modifiers
    let (input, attribute_lists) = delimited(ws, parse_attribute_lists, ws).parse(input)?;
    let attributes = convert_attributes(attribute_lists);
    let (input, modifiers) = delimited(ws, parse_modifiers, ws).parse(input)?;

    // Parse the get keyword
    let (input, _) = delimited(ws, kw_get(), ws)
        .context("get keyword")
        .parse(input)?;

    // Parse the body (either block or semicolon)
    let (input, body) = alt((
        // Semicolon (auto-accessor)
        map(
            delimited(ws, tok_semicolon(), ws)
                .context("get accessor semicolon"),
            |_| None,
        ),
        // Block body
        map(
            cut(parse_block_statement).context("get accessor body"),
            Some,
        ),
    ))
        .parse(input)?;

    Ok((
        input,
        IndexerAccessor {
            modifiers,
            attributes,
            body,
        },
    ))
}

/// Parse a set accessor declaration  
fn parse_set_accessor_declaration(input: Span) -> BResult<IndexerAccessor> {
    // Optional attribute lists and modifiers
    let (input, attribute_lists) = delimited(ws, parse_attribute_lists, ws).parse(input)?;
    let attributes = convert_attributes(attribute_lists);
    let (input, modifiers) = delimited(ws, parse_modifiers, ws).parse(input)?;

    // Parse the set keyword
    let (input, _) = delimited(ws, kw_set(), ws)
        .context("set keyword")
        .parse(input)?;

    // Parse the body (either block or semicolon)
    let (input, body) = alt((
        // Semicolon (auto-accessor)
        map(
            delimited(ws, tok_semicolon(), ws)
                .context("set accessor semicolon"),
            |_| None,
        ),
        // Block body
        map(
            cut(parse_block_statement).context("set accessor body"),
            Some,
        ),
    ))
        .parse(input)?;

    Ok((
        input,
        IndexerAccessor {
            modifiers,
            attributes,
            body,
        },
    ))
}
use crate::syntax::span::Span;
use crate::tokens::separators::{tok_comma, tok_semicolon};
