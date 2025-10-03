use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::tuple,
};

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::expressions::statements::block_statement_parser::{
    extract_statements_from_block, parse_block_statement,
};
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::declaration_keywords::kw_delegate;
use crate::parser::keywords::modifier_keywords::kw_async;
use crate::parser::keywords::parameter_modifier_keywords::{kw_in, kw_out, kw_ref};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::lambda_expression::{
    AnonymousMethodExpression, LambdaBody, LambdaExpression, LambdaParameter,
    LambdaParameterModifier,
};
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, bws, context, parse_delimited_list0};

/// Parse a lambda parameter modifier (ref, out, in)
fn parse_lambda_parameter_modifier(input: &str) -> BResult<&str, LambdaParameterModifier> {
    context(
        "lambda parameter modifier",
        alt((
            map(kw_ref(), |_| LambdaParameterModifier::Ref),
            map(kw_out(), |_| LambdaParameterModifier::Out),
            map(kw_in(), |_| LambdaParameterModifier::In),
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
                tuple((bws(parse_type_expression), bws(parse_identifier))),
                |(ty, name)| LambdaParameter {
                    name,
                    ty: Some(ty),
                    modifier: None,
                },
            ),
            // Try parameter with just modifier: ref x
            map(
                tuple((bws(parse_lambda_parameter_modifier), bws(parse_identifier))),
                |(modifier, name)| LambdaParameter {
                    name,
                    ty: None,
                    modifier: Some(modifier),
                },
            ),
            // Just identifier: x
            map(bws(parse_identifier), |name| LambdaParameter {
                name,
                ty: None,
                modifier: None,
            }),
        )),
    )(input)
}

/// Parse lambda parameters - either single parameter or parenthesized list
fn parse_lambda_parameters(input: &str) -> BResult<&str, Vec<LambdaParameter>> {
    context(
        "lambda parameters",
        alt((
            // Parenthesized list: (x, y) => x + y or (int x, string y) => ...
            parse_delimited_list0::<_, _, _, _, char, LambdaParameter, char, char, LambdaParameter>(
                bchar('('),
                parse_lambda_parameter,
                bchar(','),
                bchar(')'),
                false, // trailing comma not allowed in lambda parameter list
                true,  // cut on close
            ),
            // Single parameter without parentheses: x => x * 2
            // This should only work if there's no type or modifier
            map(bws(parse_identifier), |name| {
                vec![LambdaParameter {
                    name,
                    ty: None,
                    modifier: None,
                }]
            }),
        )),
    )(input)
}

/// Parse lambda body block statements (for lambda { ... } bodies)
fn parse_lambda_block_body(input: &str) -> BResult<&str, Vec<Statement>> {
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
            map(parse_lambda_block_body, |statements| {
                LambdaBody::Block(statements)
            }),
            // Expression body: expression
            map(parse_expression, LambdaBody::ExpressionSyntax),
        )),
    )(input)
}

/// Parse a lambda expression: \[async] parameters => body
pub fn parse_lambda_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "lambda expression",
        map(
            tuple((
                opt(kw_async()),
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
                opt(bws(kw_async())),
                bws(kw_delegate()),
                opt(parse_delimited_list0::<
                    _,
                    _,
                    _,
                    _,
                    char,
                    LambdaParameter,
                    char,
                    char,
                    LambdaParameter,
                >(
                    bchar('('),
                    parse_lambda_parameter,
                    bchar(','),
                    bchar(')'),
                    false,
                    true,
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
        alt((parse_lambda_expression, parse_anonymous_method_expression)),
    )(input)
}
