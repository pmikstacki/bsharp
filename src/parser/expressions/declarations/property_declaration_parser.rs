use crate::parser::expressions::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{PropertyAccessor, PropertyDeclaration};
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::parser_helpers::{bchar, bws, context};
use nom::bytes::complete::tag_no_case;
use nom::combinator::cut;
use nom::{
    branch::alt,
    bytes::complete::take_until,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded, tuple}, // Keep for internal nom usage if any
};

// Parse get accessor
fn parse_get_accessor(input: &str) -> BResult<&str, PropertyAccessor> {
    // Parse "get" followed by optional body or semicolon
    let (input, _) = context(
        "get accessor keyword (expected 'get')",
        bws(tag_no_case("get")),
    )(input)?;

    // Parse body or just a semicolon
    let (input, body) = alt((
        // Body with braces: "get { return _value; }"
        map(
            delimited(
                context("get accessor body opening (expected '{')", bws(bchar('{'))),
                take_until("}"),
                context("get accessor body closing (expected '}')", bws(bchar('}'))),
            ),
            |content: &str| Some(content.trim().to_string()),
        ),
        // Auto-property with just semicolon: "get;"
        map(
            context("get accessor terminator (expected ';')", bws(bchar(';'))),
            |_| None,
        ),
    ))(input)?;

    Ok((input, PropertyAccessor::Get(body)))
}

// Parse set accessor
fn parse_set_accessor(input: &str) -> BResult<&str, PropertyAccessor> {
    // Parse "set" followed by optional body or semicolon
    let (input, _) = context(
        "set accessor keyword (expected 'set')",
        bws(tag_no_case("set")),
    )(input)?;

    // Parse body or just a semicolon
    let (input, body) = alt((
        // Body with braces: "set { _value = value; }"
        map(
            delimited(
                context("set accessor body opening (expected '{')", bws(bchar('{'))),
                take_until("}"),
                context("set accessor body closing (expected '}')", bws(bchar('}'))),
            ),
            |content: &str| Some(content.trim().to_string()),
        ),
        // Auto-property with just semicolon: "set;"
        map(
            context("set accessor terminator (expected ';')", bws(bchar(';'))),
            |_| None,
        ),
    ))(input)?;

    Ok((input, PropertyAccessor::Set(body)))
}

// Parse init accessor (C# 9+)
fn parse_init_accessor(input: &str) -> BResult<&str, PropertyAccessor> {
    // Parse "init" followed by optional body or semicolon
    let (input, _) = context(
        "init accessor keyword (expected 'init')",
        bws(tag_no_case("init")),
    )(input)?;

    // Parse body or just a semicolon
    let (input, body) = alt((
        // Body with braces: "init { _value = value; }"
        map(
            delimited(
                context("init accessor body opening (expected '{')", bws(bchar('{'))),
                take_until("}"),
                context("init accessor body closing (expected '}')", bws(bchar('}'))),
            ),
            |content: &str| Some(content.trim().to_string()),
        ),
        // Auto-property with just semicolon: "init;"
        map(
            context("init accessor terminator (expected ';')", bws(bchar(';'))),
            |_| None,
        ),
    ))(input)?;

    Ok((input, PropertyAccessor::Init(body)))
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
    let (input, accessors) = context(
        "property accessors (expected accessor block with get/set/init)",
        bws(parse_property_accessors),
    )(input)?;
    let (input, initializer) = bws(parse_property_initializer)(input)?;

    Ok((
        input,
        PropertyDeclaration {
            modifiers,
            ty,
            name,
            accessors,
            initializer,
        },
    ))
}
