use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
};

use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::expressions::lambda_expression::{AnonymousMethodExpression, LambdaBody, LambdaExpression, LambdaParameter, LambdaParameterModifier};
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::statements::block_statement_parser::parse_block_statement;
use crate::parsers::types::type_parser::parse_type_expression;

/// Parse a lambda parameter modifier (ref, out, in)
fn parse_lambda_parameter_modifier(input: &str) -> BResult<&str, LambdaParameterModifier> {
    bs_context(
        "lambda parameter modifier",
        alt((
            map(keyword("ref"), |_| LambdaParameterModifier::Ref),
            map(keyword("out"), |_| LambdaParameterModifier::Out),
            map(keyword("in"), |_| LambdaParameterModifier::In),
        )),
    )(input)
}

/// Parse a lambda parameter: [modifier] [type] name
fn parse_lambda_parameter(input: &str) -> BResult<&str, LambdaParameter> {
    bs_context(
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
    bs_context(
        "lambda parameters",
        alt((
            // Empty parentheses: () => DoSomething()
            map(
                tuple((bchar('('), bws(bchar(')')))),
                |_| vec![]
            ),
            // Parenthesized list: (x, y) => x + y or (int x, string y) => ...
            delimited(
                bchar('('),
                separated_list0(bws(bchar(',')), bws(parse_lambda_parameter)),
                bws(bchar(')'))
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

/// Parse lambda body - either expression or block
fn parse_lambda_body(input: &str) -> BResult<&str, LambdaBody> {
    bs_context(
        "lambda body",
        alt((
            // Block body: { statements... }
            map(parse_block_statement, |_block| {
                // Convert Statement::Block to Vec<Expression> - simplified for now
                // In a real implementation, you'd need proper statement-to-expression conversion
                LambdaBody::Block(vec![])
            }),
            // Expression body: expression
            map(parse_expression, |expr| LambdaBody::ExpressionSyntax(expr)),
        )),
    )(input)
}

/// Parse a lambda expression: [async] parameters => body
pub fn parse_lambda_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
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
    bs_context(
        "anonymous method expression",
        map(
            tuple((
                opt(bws(keyword("async"))),
                bws(keyword("delegate")),
                opt(delimited(
                    bws(bchar('(')),
                    separated_list0(bws(bchar(',')), bws(parse_lambda_parameter)),
                    bws(bchar(')'))
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
    bs_context(
        "lambda or anonymous method",
        alt((
            parse_lambda_expression,
            parse_anonymous_method_expression,
        )),
    )(input)
} 