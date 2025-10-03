use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::exception_and_safety_keywords::kw_throw;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::throw_expression::ThrowExpression;
use crate::syntax::parser_helpers::context;

use nom::{
    combinator::{map, opt},
    sequence::preceded,
};

/// Parse a throw expression: `throw expr` or just `throw`
pub fn parse_throw_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "throw expression",
        map(
            preceded(
                kw_throw(),
                opt(preceded(
                    nom::character::complete::multispace1,
                    parse_expression,
                )),
            ),
            |expr_opt| {
                Expression::Throw(Box::new(ThrowExpression {
                    expr: expr_opt.map(Box::new),
                }))
            },
        ),
    )(input)
}
