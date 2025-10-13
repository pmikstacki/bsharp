use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;
use nom::{
    combinator::{map, opt},
    sequence::{preceded, tuple},
};
use nom::Parser;
use nom::character::complete::satisfy;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;
use syntax::declarations::local_variable_declaration::VariableDeclarator;
use syntax::declarations::LocalVariableDeclaration;

/// Parse a variable declarator (name with optional initializer)
/// Example: "x = 5" or just "x"
pub fn parse_variable_declarator(input: Span) -> BResult<VariableDeclarator> {
    map(
        tuple((
            delimited(ws, parse_identifier, ws)
                .context("variable name"),
            opt(preceded(
                delimited(ws, satisfy(|c| c == '='), ws)
                    .context("variable initializer"),
                delimited(ws, parse_expression, ws)
                    .context("variable initializer expression"),
            )),
        )),
        |(name, initializer)| VariableDeclarator { name, initializer },
    )
    .parse(input)
}

/// Parse a variable declaration
/// Examples: "int x = 5", "var y", "string name, address", "const double PI = 3.14"
pub fn parse_variable_declaration(input: Span) -> BResult<LocalVariableDeclaration> {
    // Parse optional const modifier
    let (input, is_const) = map(
        opt(|i| delimited(ws, tag("const"), ws).parse(i)),
        |opt| opt.is_some(),
    )
    .context("optional const modifier")
    .parse(input)?;

    // Note: For variable declarations, we start with a type
    let (input, variable_type) = delimited(ws, parse_type_expression, ws)
        .context("variable type")
        .parse(input)?;

    // Parse one or more variable declarators separated by commas
    let (input, declarators) = separated_list1(
        |i| delimited(ws, satisfy(|c| c == ','), ws).parse(i),
        |i| delimited(ws, parse_variable_declarator, ws).parse(i),
    )
    .context("variable declarators")
    .parse(input)?;

    Ok((
        input,
        LocalVariableDeclaration {
            declaration_type: variable_type,
            declarators,
            is_const,
            is_ref: false,
        },
    ))
}

/// Parse a local variable declaration statement (with semicolon)
/// Example: "int x = 5;"
pub fn parse_local_variable_declaration<'a>(input: Span<'a>) -> BResult<'a, LocalVariableDeclaration> {
    let (input, declaration) = parse_variable_declaration(input)?;

    let (input, _) = delimited(ws, satisfy(|c| c == ';'), ws)
        .context("variable declaration terminator")
        .parse(input)?;

    Ok((input, declaration))
}

/// Wrapper function to use in statement parsing
pub fn parse_local_variable_declaration_statement(
    input: Span,
) -> BResult<crate::syntax::statements::statement::Statement> {
    use crate::syntax::statements::statement::Statement;
    map(parse_local_variable_declaration, Statement::Declaration)
        .parse(input)
}
use crate::syntax::span::Span;
