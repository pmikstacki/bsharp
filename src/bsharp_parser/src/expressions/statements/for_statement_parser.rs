// Parser for for loops

use crate::parser::expressions::declarations::variable_declaration_parser::parse_variable_declarator;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::iteration_keywords::kw_for;
use crate::parser::keywords::modifier_keywords::kw_const;
use crate::parser::statement_parser::parse_statement_ws;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::list_parser::parse_list0;
use nom::combinator::cut;
use nom::Parser;
use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::delimited,
};
use nom_supreme::ParserExt;
use syntax::declarations::LocalVariableDeclaration;
use syntax::statements::statement::Statement;
use syntax::statements::{ForInitializer, ForStatement};

// Parse the initializer part of a for loop statement - can be a variable declaration
// or a comma-separated list of expressions
fn parse_for_initializer(input: Span) -> BResult<ForInitializer> {
    alt((
        // Try to parse a variable declaration first (e.g., "int i = 0")
        map(
            (
                // Optionally parse "const"
                opt(delimited(ws, kw_const(), ws))
                    .context("optional const modifier"),
                // Parse type name
                delimited(ws, parse_type_expression, ws)
                    .context("variable type"),
                // Parse declarators (name and initializer)
                separated_list1(
                    delimited(ws, tok_comma(), ws),
                    parse_variable_declarator,
                )
                    .context("variable declarators"),
            ),
            |(const_modifier, ty, declarators)| {
                ForInitializer::Declaration(LocalVariableDeclaration {
                    is_const: const_modifier.is_some(),
                    is_ref: false, // For now, ref locals in for loops are not supported
                    declaration_type: ty,
                    declarators,
                })
            },
        ),
        // If that fails, try to parse expressions (e.g., "i = 0, j = 1")
        map(
            separated_list1(
                delimited(ws, tok_comma(), ws),
                delimited(ws, parse_expression, ws),
            )
                .context("expression list"),
            ForInitializer::Expressions,
        ),
    ))
        .context("for loop initializer")
        .parse(input)
}

// Original parse_for_statement function from statement_parser.rs
// Parse a for loop statement using Roslyn-like structure
pub fn parse_for_statement(input: Span) -> BResult<Statement> {
    (|input| {
        let (input, _) = kw_for().context("for keyword").parse(input)?;
        let (input, _) = delimited(ws, tok_l_paren(), ws)
            .context("opening parenthesis after for")
            .parse(input)?;

        // 1. Parse initializer (optional) then semicolon
        let (input, initializer) = opt(delimited(ws, parse_for_initializer, ws))
            .context("for loop initializer")
            .parse(input)?;
        let (input, _) = delimited(ws, tok_semicolon(), ws)
            .context("for loop semicolon")
            .parse(input)?;

        // 2. Parse condition (optional) then semicolon
        let (input, condition) = opt(delimited(ws, parse_expression, ws))
            .context("for loop condition")
            .parse(input)?;
        let (input, _) = delimited(ws, tok_semicolon(), ws)
            .context("for loop semicolon")
            .parse(input)?;

        // 3. Parse iterators list (comma-separated expressions)
        let (input, iterators) = parse_list0(parse_expression, tok_comma())
            .context("for loop iterators")
            .parse(input)?;

        // 4. Parse closing parenthesis
        let (input, _) = cut(delimited(ws, tok_r_paren(), ws))
            .context("for loop closing parenthesis")
            .parse(input)?;

        // 5. Parse body statement
        let (input, body) = cut(delimited(ws, parse_statement_ws, ws))
            .context("for loop body ")
            .parse(input)?;

        Ok((
            input,
            Statement::For(Box::new(ForStatement {
                initializer,
                condition,
                iterator: iterators,
                body: Box::new(body),
            })),
        ))
    })
        .context("for statement")
        .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
use crate::tokens::separators::{tok_comma, tok_semicolon};
