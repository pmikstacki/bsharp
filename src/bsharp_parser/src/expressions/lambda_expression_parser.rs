use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::expressions::statements::block_statement_parser::{
    extract_statements_from_block, parse_block_statement,
};
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::declaration_keywords::kw_delegate;
use crate::parser::keywords::modifier_keywords::kw_async;
use crate::parser::keywords::parameter_modifier_keywords::{kw_in, kw_out, kw_ref};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::list_parser::parse_delimited_list0;
use nom::character::complete::satisfy;
use nom::sequence::delimited;
use nom::Parser;
use nom::{
    branch::alt,
    combinator::{map, opt},
};
use nom_supreme::ParserExt;
use syntax::expressions::{
    AnonymousMethodExpression, Expression, LambdaBody, LambdaExpression, LambdaParameter,
    LambdaParameterModifier,
};
use syntax::statements::statement::Statement;

/// Parse a parenthesized lambda parameter list: (p1, p2, ...)
fn parse_param_list(i: Span) -> BResult<Vec<LambdaParameter>> {
    parse_delimited_list0::<_, _, _, _, char, char, char, LambdaParameter>(
        |i| delimited(ws, tok_l_paren(), ws).parse(i),
        |i| delimited(ws, parse_lambda_parameter, ws).parse(i),
        |i| delimited(ws, tok_comma(), ws).parse(i),
        |i| delimited(ws, tok_r_paren(), ws).parse(i),
        false,
        true,
    )
        .parse(i)
}

/// Parse a lambda parameter modifier (ref, out, in)
fn parse_lambda_parameter_modifier(input: Span) -> BResult<LambdaParameterModifier> {
    map(
        alt((
            map(kw_ref(), |_| LambdaParameterModifier::Ref),
            map(kw_out(), |_| LambdaParameterModifier::Out),
            map(kw_in(), |_| LambdaParameterModifier::In),
        )),
        |v| v,
    )
        .context("lambda parameter modifier")
        .parse(input)
}

/// Parse a lambda parameter: [modifier] [type] name
fn parse_lambda_parameter(input: Span) -> BResult<LambdaParameter> {
    map(alt((
        // Try full parameter with modifier and type: ref int x
        map(
            (
                delimited(ws, parse_lambda_parameter_modifier, ws),
                delimited(ws, parse_type_expression, ws),
                delimited(ws, parse_identifier, ws),
            ),
            |(modifier, ty, name)| LambdaParameter {
                name,
                ty: Some(ty),
                modifier: Some(modifier),
            },
        ),
        // Try parameter with just type: int x
        map(
            (delimited(ws, parse_type_expression, ws), delimited(ws, parse_identifier, ws)),
            |(ty, name)| LambdaParameter {
                name,
                ty: Some(ty),
                modifier: None,
            },
        ),
        // Try parameter with just modifier: ref x
        map(
            (delimited(ws, parse_lambda_parameter_modifier, ws), delimited(ws, parse_identifier, ws)),
            |(modifier, name)| LambdaParameter {
                name,
                ty: None,
                modifier: Some(modifier),
            },
        ),
        // Just identifier: x
        map(delimited(ws, parse_identifier, ws), |name| LambdaParameter {
            name,
            ty: None,
            modifier: None,
        }),
    )), |v| v)
        .context("lambda parameter")
        .parse(input)
}

/// Parse lambda parameters - either single parameter or parenthesized list
fn parse_lambda_parameters(input: Span) -> BResult<Vec<LambdaParameter>> {
    map(alt((
        // Parenthesized list: (x, y) => x + y or (int x, string y) => ...
        parse_param_list,
        // Single parameter without parentheses: x => x * 2
        // This should only work if there's no type or modifier
        map(delimited(ws, parse_identifier, ws), |name| {
            vec![LambdaParameter {
                name,
                ty: None,
                modifier: None,
            }]
        }),
    )), |v| v)
        .context("lambda parameters")
        .parse(input)
}

/// Parse lambda body block statements (for lambda { ... } bodies)
fn parse_lambda_block_body(input: Span) -> BResult<Vec<Statement>> {
    let (input, block_statement) = parse_block_statement(input)?;
    let statements = extract_statements_from_block(block_statement);
    Ok((input, statements))
}

/// Parse lambda body - either expression or block
fn parse_lambda_body(input: Span) -> BResult<LambdaBody> {
    map(alt((
        // Block body: { statements... }
        map(parse_lambda_block_body, |statements| {
            LambdaBody::Block(statements)
        }),
        // Expression body: expression
        map(parse_expression, LambdaBody::ExpressionSyntax),
    )), |v| v)
        .context("lambda body")
        .parse(input)
}

/// Parse a lambda expression: \[async] parameters => body
pub fn parse_lambda_expression(input: Span) -> BResult<Expression> {
    map(
        (
            opt(kw_async()),
            delimited(ws, parse_lambda_parameters, ws),
            delimited(ws, tok_assign(), ws),
            delimited(ws, satisfy(|c| c == '>'), ws),
            delimited(ws, parse_lambda_body, ws),
        ),
        |(async_kw, parameters, _, _, body)| {
            Expression::Lambda(Box::new(LambdaExpression {
                parameters,
                body,
                is_async: async_kw.is_some(),
            }))
        },
    )
        .context("lambda expression")
        .parse(input)
}

/// Parse an anonymous method expression: async delegate [parameters] body
pub fn parse_anonymous_method_expression(input: Span) -> BResult<Expression> {
    map(
        (
            opt(|i| delimited(ws, kw_async(), ws).parse(i)),
            delimited(ws, kw_delegate(), ws),
            opt(parse_param_list),
            delimited(ws, parse_lambda_body, ws),
        ),
        |(async_kw, _, parameters, body)| {
            Expression::AnonymousMethod(Box::new(AnonymousMethodExpression {
                parameters: parameters.unwrap_or_default(),
                body,
                is_async: async_kw.is_some(),
            }))
        },
    )
        .context("anonymous method expression")
        .parse(input)
}

/// Parse any lambda-like expression (lambda or anonymous method)
pub fn parse_lambda_or_anonymous_method(input: Span) -> BResult<Expression> {
    map(alt((parse_lambda_expression, parse_anonymous_method_expression)), |v| v)
        .context("lambda or anonymous method")
        .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::assignment::tok_assign;
use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
use crate::tokens::separators::tok_comma;
