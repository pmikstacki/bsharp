use nom::{
    branch::alt,
    character::complete::alpha1,
    combinator::{map, not, opt, peek},
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
};
use nom_supreme::tag::complete::tag;
use nom::combinator::cut;

use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::lambda_expression::{AnonymousMethodExpression, LambdaBody, LambdaExpression, LambdaParameter, LambdaParameterModifier};
use crate::syntax::parser_helpers::{bchar, context, bws, keyword};
use crate::parser::expressions::expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::parser::statements::block_statement_parser::{parse_block_statement, extract_statements_from_block};

// Helper to ensure we match complete words, not prefixes
fn word_boundary(input: &str) -> BResult<&str, ()> {
    // Check that the next character is not alphanumeric or underscore, without consuming it
    peek(not(alpha1))(input)
}

/// Parse a lambda parameter modifier (ref, out, in)
fn parse_lambda_parameter_modifier(input: &str) -> BResult<&str, LambdaParameterModifier> {
    context(
        "lambda parameter modifier",
        alt((
            map(terminated(tag("ref"), word_boundary), |_| LambdaParameterModifier::Ref),
            map(terminated(tag("out"), word_boundary), |_| LambdaParameterModifier::Out),
            map(terminated(tag("in"), word_boundary), |_| LambdaParameterModifier::In),
        )),
    )(input)
}

/// Parse a lambda parameter: [modifier] [type] name
fn parse_lambda_parameter(input: &str) -> BResult<&str, LambdaParameter> {
    context(
        "lambda parameter",
        alt((
            // Try full parameter with modifier and type: ref int x
            map(
                tuple((
                    bws(parse_lambda_parameter_modifier),
                    bws(parse_type_expression),
                    bws(parse_identifier),
                )),
                |(modifier, ty, name)| LambdaParameter {
                    name,
                    ty: Some(ty),
                    modifier: Some(modifier),
                },
            ),
            // Try parameter with just type: int x
            map(
                tuple((
                    bws(parse_type_expression),
                    bws(parse_identifier),
                )),
                |(ty, name)| LambdaParameter {
                    name,
                    ty: Some(ty),
                    modifier: None,
                },
            ),
            // Try parameter with just modifier: ref x
            map(
                tuple((
                    bws(parse_lambda_parameter_modifier),
                    bws(parse_identifier),
                )),
                |(modifier, name)| LambdaParameter {
                    name,
                    ty: None,
                    modifier: Some(modifier),
                },
            ),
            // Just identifier: x
            map(
                bws(parse_identifier),
                |name| LambdaParameter {
                    name,
                    ty: None,
                    modifier: None,
                },
            ),
        )),
    )(input)
}

/// Parse lambda parameters - either single parameter or parenthesized list
fn parse_lambda_parameters(input: &str) -> BResult<&str, Vec<LambdaParameter>> {
    context(
        "lambda parameters",
        alt((
            // Parenthesized list: (x, y) => x + y or (int x, string y) => ...
            delimited(
                bchar('('),
                separated_list0(bws(bchar(',')), bws(parse_lambda_parameter)),
                cut(bws(bchar(')')))
            ),
            // Single parameter without parentheses: x => x * 2
            // This should only work if there's no type or modifier
            map(
                bws(parse_identifier),
                |name| vec![LambdaParameter {
                    name,
                    ty: None,
                    modifier: None,
                }]
            ),
        )),
    )(input)
}

/// Parse lambda body block statements (for lambda { ... } bodies)
fn parse_lambda_block_body(input: &str) -> BResult<&str, Vec<crate::syntax::nodes::statements::statement::Statement>> {
    let (input, block_statement) = parse_block_statement(input)?;
    let statements = extract_statements_from_block(block_statement);
    Ok((input, statements))
}

/// Parse lambda body - either expression or block
fn parse_lambda_body(input: &str) -> BResult<&str, LambdaBody> {
    context(
        "lambda body",
        alt((
            // Block body: { statements... }
            // We need our own block syntax here to avoid recursion issues
            map(parse_lambda_block_body, |statements| LambdaBody::Block(statements)),
            // Expression body: expression
            map(parse_expression, |expr| LambdaBody::ExpressionSyntax(expr)),
        )),
    )(input)
}

/// Parse a lambda expression: [async] parameters => body
pub fn parse_lambda_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "lambda expression",
        map(
            tuple((
                opt(keyword("async")),
                bws(parse_lambda_parameters),
                bws(bchar('=')), 
                bws(bchar('>')),
                bws(parse_lambda_body),
            )),
            |(async_kw, parameters, _, _, body)| {
                Expression::Lambda(Box::new(LambdaExpression {
                    parameters,
                    body,
                    is_async: async_kw.is_some(),
                }))
            },
        ),
    )(input)
}

/// Parse an anonymous method expression: [async] delegate [parameters] body
pub fn parse_anonymous_method_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "anonymous method expression",
        map(
            tuple((
                opt(bws(keyword("async"))),
                bws(keyword("delegate")),
                opt(delimited(
                    bws(bchar('(')),
                    separated_list0(bws(bchar(',')), bws(parse_lambda_parameter)),
                    cut(bws(bchar(')')))
                )),
                bws(parse_lambda_body),
            )),
            |(async_kw, _, parameters, body)| {
                Expression::AnonymousMethod(Box::new(AnonymousMethodExpression {
                    parameters: parameters.unwrap_or_default(),
                    body,
                    is_async: async_kw.is_some(),
                }))
            },
        ),
    )(input)
}

/// Parse any lambda-like expression (lambda or anonymous method)
pub fn parse_lambda_or_anonymous_method(input: &str) -> BResult<&str, Expression> {
    context(
        "lambda or anonymous method",
        alt((
            parse_lambda_expression,
            parse_anonymous_method_expression,
        )),
    )(input)
} 