use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::satisfy;
use nom::combinator::cut;
use nom::sequence::delimited;
use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::many0,
    sequence::preceded,
};
use nom_supreme::ParserExt;
use syntax::declarations::{PropertyAccessor, PropertyDeclaration};
use syntax::expressions::Expression;
use syntax::statements::statement::Statement;

// Parse get accessor
fn parse_get_accessor(input: Span) -> BResult<PropertyAccessor> {
    // Optional attribute lists and modifiers
    let (input, attributes) = delimited(ws, parse_attribute_lists, ws).parse(input)?;
    let (input, modifiers) =
        delimited(ws, |i| parse_modifiers_for_decl_type(i, "property"), ws).parse(input)?;
    // Parse "get" followed by optional body or semicolon
    let (input, _) = delimited(ws, tag_no_case("get"), ws)
        .context("get accessor keyword")
        .parse(input)?;

    // Parse body or just a semicolon
    let (input, body) = map(alt((
        // Expression-bodied accessor: get => expr;
        map(
            (
                delimited(ws, tag_no_case("=>"), ws),
                cut((
                    delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
                    delimited(ws, tok_semicolon(), ws),
                )),
            ),
            |(_, (expr, _))| Some(Statement::Expression(expr)),
        ),
        // Block body: get { ... }
        map(
            (|i| crate::parser::expressions::statements::block_statement_parser::parse_block_statement(i))
                .context("get accessor body"),
            Some,
        ),
        // Auto-property with just semicolon: "get;"
        map(
            delimited(ws, tok_semicolon(), ws)
                .context("get accessor terminator"),
            |_| None,
        ),
    )), |x| x)
        .parse(input)?;

    Ok((
        input,
        PropertyAccessor::Get {
            modifiers,
            attributes,
            body,
        },
    ))
}

// Parse set accessor
fn parse_set_accessor(input: Span) -> BResult<PropertyAccessor> {
    // Optional attribute lists and modifiers
    let (input, attributes) = delimited(ws, parse_attribute_lists, ws).parse(input)?;
    let (input, modifiers) =
        delimited(ws, |i| parse_modifiers_for_decl_type(i, "property"), ws).parse(input)?;
    // Parse "set" followed by optional body or semicolon
    let (input, _) = delimited(ws, tag_no_case("set"), ws)
        .context("set accessor keyword (expected 'set')")
        .parse(input)?;

    // Parse body or just a semicolon
    let (input, body) = map(alt((
        // Expression-bodied accessor: set => expr;
        map(
            (
                delimited(ws, tag_no_case("=>"), ws),
                cut((
                    delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
                    delimited(ws, tok_semicolon(), ws),
                )),
            ),
            |(_, (expr, _))| Some(Statement::Expression(expr)),
        ),
        map(
            (|i| crate::parser::expressions::statements::block_statement_parser::parse_block_statement(i))
                .context("set accessor body"),
            Some,
        ),
        map(
            delimited(ws, tok_semicolon(), ws)
                .context("set accessor terminator"),
            |_| None,
        ),
    )), |x| x)
        .parse(input)?;

    Ok((
        input,
        PropertyAccessor::Set {
            modifiers,
            attributes,
            body,
        },
    ))
}

// Parse init accessor (C# 9+)
fn parse_init_accessor(input: Span) -> BResult<PropertyAccessor> {
    // Optional attribute lists and modifiers
    let (input, attributes) = delimited(ws, parse_attribute_lists, ws).parse(input)?;
    let (input, modifiers) =
        delimited(ws, |i| parse_modifiers_for_decl_type(i, "property"), ws).parse(input)?;
    // Parse "init" followed by optional body or semicolon
    let (input, _) = delimited(ws, tag_no_case("init"), ws)
        .context("init accessor keyword (expected 'init')")
        .parse(input)?;

    // Parse body or just a semicolon
    let (input, body) = map(alt((
        // Expression-bodied accessor: init => expr;
        map(
            (
                delimited(ws, tag_no_case("=>"), ws),
                cut((
                    delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
                    delimited(ws, tok_semicolon(), ws),
                )),
            ),
            |(_, (expr, _))| Some(Statement::Expression(expr)),
        ),
        map(
            (|i| crate::parser::expressions::statements::block_statement_parser::parse_block_statement(i))
                .context("init accessor body"),
            Some,
        ),
        map(
            delimited(ws, tok_semicolon(), ws)
                .context("init accessor terminator"),
            |_| None,
        ),
    )), |x| x)
        .parse(input)?;

    Ok((
        input,
        PropertyAccessor::Init {
            modifiers,
            attributes,
            body,
        },
    ))
}

// Parse property accessors - can be one or more accessors in braces
fn parse_property_accessors(input: Span) -> BResult<Vec<PropertyAccessor>> {
    delimited(
        delimited(ws, satisfy(|c| c == '{'), ws).context("property accessors opening"),
        many0(|i| {
            if let Ok(r) = parse_get_accessor(i) {
                return Ok(r);
            }
            if let Ok(r) = parse_set_accessor(i) {
                return Ok(r);
            }
            parse_init_accessor(i)
        }),
        cut(delimited(ws, satisfy(|c| c == '}'), ws)).context("property accessors closing"),
    )
    .parse(input)
}

// Parse optional property initializer: " = expression;"
fn parse_property_initializer(input: Span) -> BResult<Option<Expression>> {
    opt(preceded(
        delimited(ws, tok_assign(), ws).context("property initializer"),
        (
            delimited(ws, parse_expression_spanned, ws)
                .map(|s| s.node)
                .context("property initializer expression"),
            delimited(ws, tok_semicolon(), ws).context("property initializer terminator"),
        ),
    ))
    .parse(input)
    .map(|(input, result)| (input, result.map(|(expr, _)| expr)))
}

// Parse a property declaration
pub fn parse_property_declaration(input: Span) -> BResult<PropertyDeclaration> {
    // Parse attributes (zero or more groups)
    let (input, attributes) = delimited(ws, parse_attribute_lists, ws).parse(input)?;
    // Parse modifiers specifically for property declarations (they consume trailing space)
    let (input, modifiers) = (|i| parse_modifiers_for_decl_type(i, "property"))
        .context("property modifiers")
        .parse(input)?;
    // Consume any additional optional whitespace before the type
    let (input, _) = ws(input)?;
    let (input, ty) = parse_type_expression
        .context("property type")
        .parse(input)?;
    let (input, name) = delimited(ws, parse_identifier, ws)
        .context("property name")
        .parse(input)?;
    // Either an accessor block { ... } or an expression-bodied property => expr;
    let (input, (accessors, initializer)) = map(
        alt((
            // Expression-bodied property: `=> expr;`
            map(
                (
                    delimited(ws, tag_no_case("=>"), ws),
                    cut((
                        delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
                        delimited(ws, tok_semicolon(), ws),
                    )),
                ),
                |(_, (expr, _))| {
                    (
                        vec![PropertyAccessor::Get {
                            modifiers: vec![],
                            attributes: vec![],
                            body: Some(Statement::Expression(expr)),
                        }],
                        None,
                    )
                },
            ),
            // Traditional accessor block with optional initializer
            map(
                (
                    |i| delimited(ws, parse_property_accessors, ws).parse(i),
                    |i| delimited(ws, parse_property_initializer, ws).parse(i),
                ),
                |(accessors, initializer)| (accessors, initializer),
            ),
        )),
        |x| x,
    )
    .parse(input)?;

    Ok((
        input,
        PropertyDeclaration {
            attributes,
            modifiers,
            property_type: ty,
            name,
            accessors,
            initializer,
        },
    ))
}
use syntax::span::Span;

use crate::tokens::assignment::tok_assign;
use crate::tokens::separators::tok_semicolon;
