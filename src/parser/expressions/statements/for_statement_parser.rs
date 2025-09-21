// Parser for for loops

use nom::combinator::cut;
use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::tuple,
};

use crate::parser::expressions::declarations::variable_declaration_parser::parse_variable_declarator;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement_ws;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::local_variable_declaration::LocalVariableDeclaration;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::statements::{ForInitializer, ForStatement};
use crate::syntax::parser_helpers::{bchar, bws, context, keyword, parse_list0};
use crate::parser::keywords::iteration_keywords::kw_for;

// Parse the initializer part of a for loop statement - can be a variable declaration
// or a comma-separated list of expressions
fn parse_for_initializer(input: &str) -> BResult<&str, ForInitializer> {
    context(
        "for loop initializer (expected variable declaration like 'int i = 0' or expressions like 'i = 0, j = 1')",
        alt((
            // Try to parse a variable declaration first (e.g., "int i = 0")
            // Note: For a for loop, don't expect a semicolon at the end of the variable declaration
            map(
                tuple((
                    // Optionally parse "const"
                    context(
                        "optional const modifier (expected 'const' or type name)",
                        opt(keyword("const")),
                    ),
                    // Parse type name
                    context(
                        "variable type (expected valid C# type)",
                        bws(parse_type_expression),
                    ),
                    // Parse declarators (name and initializer)
                    context(
                        "variable declarators (expected variable name and optional initializer)",
                        separated_list1(bws(bchar(',')), parse_variable_declarator),
                    ),
                )),
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
                context(
                    "expression list (expected comma-separated expressions)",
                    separated_list1(bws(bchar(',')), bws(parse_expression)),
                ),
                |exprs| ForInitializer::Expressions(exprs),
            ),
        )),
    )(input)
}

// Original parse_for_statement function from statement_parser.rs
// Parse a for loop statement using Roslyn-like structure
pub fn parse_for_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "for statement (expected 'for (initializer; condition; iterator) body')",
        |input| {
            // Base approach addressing all the test cases
            let (input, _) = context("for keyword (expected 'for')", kw_for())(input)?;
            let (input, _) = context(
                "opening parenthesis after for (expected '(')",
                bws(bchar('(')),
            )(input)?;

            // 1. Parse initializer (optional)
            let (input, initializer) = context(
                "for loop initializer section (expected variable declaration or expressions, followed by ';')",
                opt(bws(parse_for_initializer)),
            )(input)?;
            let (input, _) = context(
                "semicolon after for initializer (expected ';')",
                bws(bchar(';')),
            )(input)?;

            // 2. Parse condition (optional)
            let (input, condition) = context(
                "for loop condition section (expected boolean expression or empty, followed by ';')",
                opt(bws(parse_expression)),
            )(input)?;
            let (input, _) = context(
                "semicolon after for condition (expected ';')",
                bws(bchar(';')),
            )(input)?;

            // 3. Parse iterators - the critical part for the test case
            // We need to handle multiple comma-separated expressions
            let (input, iterators) = context(
                "for loop iterator section (expected comma-separated expressions)",
                parse_list0(parse_expression, bchar(',')),
            )(input)?;

            // 4. Parse closing parenthesis
            let (input, _) = context(
                "closing parenthesis after for header (expected ')')",
                cut(bws(bchar(')'))),
            )(input)?;

            // 5. Parse body statement, ensuring we handle comments correctly
            let (input, body) = context(
                "for loop body (expected valid C# statement)",
                bws(parse_statement_ws),
            )(input)?;

            // Create and return the ForStatement node
            Ok((
                input,
                Statement::For(Box::new(ForStatement {
                    initializer,
                    condition,
                    iterator: iterators,
                    body: Box::new(body),
                })),
            ))
        },
    )(input)
}
