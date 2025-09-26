use crate::parser::expressions::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{PropertyAccessor, PropertyDeclaration};
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::parser_helpers::{bchar, bws, context};
use nom::bytes::complete::tag_no_case;
use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::many0,
    sequence::{tuple, delimited, preceded},
};
use nom::combinator::cut;

// Parse get accessor
fn parse_get_accessor(input: &str) -> BResult<&str, PropertyAccessor> {
    // Optional attribute lists and modifiers
    let (input, attributes) = bws(parse_attribute_lists)(input)?;
    let (input, modifiers) = bws(|i| parse_modifiers_for_decl_type(i, "property"))(input)?;
    // Parse "get" followed by optional body or semicolon
    let (input, _) = context(
        "get accessor keyword (expected 'get')",
        bws(tag_no_case("get")),
    )(input)?;

    // Parse body or just a semicolon
    let (input, body) = alt((
        // Expression-bodied accessor: get => expr;
        map(
            tuple((
                bws(tag_no_case("=>")),
                bws(parse_expression),
                bws(bchar(';')),
            )),
            |(_, expr, _)| Some(Statement::Expression(expr)),
        ),
        // Block body: get { ... }
        map(
            context("get accessor body (expected block)", crate::parser::expressions::statements::block_statement_parser::parse_block_statement),
            Some,
        ),
        // Auto-property with just semicolon: "get;"
        map(
            context("get accessor terminator (expected ';')", bws(bchar(';'))),
            |_| None,
        ),
    ))(input)?;

    Ok((input, PropertyAccessor::Get { modifiers, attributes, body }))
}

// Parse set accessor
fn parse_set_accessor(input: &str) -> BResult<&str, PropertyAccessor> {
    // Optional attribute lists and modifiers
    let (input, attributes) = bws(parse_attribute_lists)(input)?;
    let (input, modifiers) = bws(|i| parse_modifiers_for_decl_type(i, "property"))(input)?;
    // Parse "set" followed by optional body or semicolon
    let (input, _) = context(
        "set accessor keyword (expected 'set')",
        bws(tag_no_case("set")),
    )(input)?;

    // Parse body or just a semicolon
    let (input, body) = alt((
        // Expression-bodied accessor: set => expr;
        map(
            tuple((
                bws(tag_no_case("=>")),
                bws(parse_expression),
                bws(bchar(';')),
            )),
            |(_, expr, _)| Some(Statement::Expression(expr)),
        ),
        map(
            context("set accessor body (expected block)", crate::parser::expressions::statements::block_statement_parser::parse_block_statement),
            Some,
        ),
        map(
            context("set accessor terminator (expected ';')", bws(bchar(';'))),
            |_| None,
        ),
    ))(input)?;

    Ok((input, PropertyAccessor::Set { modifiers, attributes, body }))
}

// Parse init accessor (C# 9+)
fn parse_init_accessor(input: &str) -> BResult<&str, PropertyAccessor> {
    // Optional attribute lists and modifiers
    let (input, attributes) = bws(parse_attribute_lists)(input)?;
    let (input, modifiers) = bws(|i| parse_modifiers_for_decl_type(i, "property"))(input)?;
    // Parse "init" followed by optional body or semicolon
    let (input, _) = context(
        "init accessor keyword (expected 'init')",
        bws(tag_no_case("init")),
    )(input)?;

    // Parse body or just a semicolon
    let (input, body) = alt((
        // Expression-bodied accessor: init => expr;
        map(
            tuple((
                bws(tag_no_case("=>")),
                bws(parse_expression),
                bws(bchar(';')),
            )),
            |(_, expr, _)| Some(Statement::Expression(expr)),
        ),
        map(
            context("init accessor body (expected block)", crate::parser::expressions::statements::block_statement_parser::parse_block_statement),
            Some,
        ),
        map(
            context("init accessor terminator (expected ';')", bws(bchar(';'))),
            |_| None,
        ),
    ))(input)?;

    Ok((input, PropertyAccessor::Init { modifiers, attributes, body }))
}

// Parse property accessors - can be one or more accessors in braces
fn parse_property_accessors(input: &str) -> BResult<&str, Vec<PropertyAccessor>> {
    delimited(
        context("property accessors opening (expected '{')", bws(bchar('{'))),
        many0(context(
            "property accessor (expected 'get', 'set', or 'init')",
            alt((parse_get_accessor, parse_set_accessor, parse_init_accessor)),
        )),
        context(
            "property accessors closing (expected '}')",
            cut(bws(bchar('}'))),
        ),
    )(input)
}

// Parse optional property initializer: " = expression;"
fn parse_property_initializer(input: &str) -> BResult<&str, Option<Expression>> {
    opt(preceded(
        context("property initializer (expected '=')", bws(bchar('='))),
        tuple((
            context(
                "property initializer expression (expected valid C# expression)",
                bws(parse_expression),
            ),
            context(
                "property initializer terminator (expected ';')",
                bws(bchar(';')),
            ),
        )),
    ))(input)
    .map(|(input, result)| (input, result.map(|(expr, _)| expr)))
}

// Parse a property declaration
pub fn parse_property_declaration(input: &str) -> BResult<&str, PropertyDeclaration> {
    // Parse attributes (zero or more groups)
    let (input, attributes) = bws(parse_attribute_lists)(input)?;
    // Parse modifiers specifically for property declarations (they consume trailing space)
    let (input, modifiers) = context(
        "property modifiers (expected valid property modifiers)",
        |i| parse_modifiers_for_decl_type(i, "property"),
    )(input)?;
    // Consume any additional optional whitespace before the type
    let (input, _) = ws(input)?;
    let (input, ty) = context(
        "property type (expected valid type expression)",
        parse_type_expression,
    )(input)?;
    let (input, name) = context(
        "property name (expected valid identifier)",
        bws(parse_identifier),
    )(input)?;
    // Either an accessor block { ... } or an expression-bodied property => expr;
    let (input, (accessors, initializer)) = alt((
        // Expression-bodied property: `=> expr;`
        map(
            tuple((
                bws(tag_no_case("=>")),
                bws(parse_expression),
                bws(bchar(';')),
            )),
            |(_, expr, _)| {
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
            tuple((bws(parse_property_accessors), bws(parse_property_initializer))),
            |(accessors, initializer)| (accessors, initializer),
        ),
    ))(input)?;

    Ok((
        input,
        PropertyDeclaration {
            attributes,
            modifiers,
            ty,
            name,
            accessors,
            initializer,
        },
    ))
}
