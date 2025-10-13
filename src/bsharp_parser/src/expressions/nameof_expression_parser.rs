use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::expression_keywords::kw_nameof;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;

use nom::combinator::cut;
use nom::{
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
};
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::expressions::{Expression, NameofExpression};

/// Parse a qualified name like "MyClass.MyMethod" or just "MyMethod"
fn parse_qualified_name(input: Span) -> BResult<Expression> {
    map(separated_list1(nom_char('.'), parse_identifier), |parts| {
        if parts.len() == 1 {
            Expression::Variable(parts.into_iter().next().unwrap())
        } else {
            // Build a chain of member accesses
            let mut expr = Expression::Variable(parts[0].clone());
            for part in parts.into_iter().skip(1) {
                expr = Expression::MemberAccess(Box::new(
                    syntax::expressions::member_access_expression::MemberAccessExpression {
                        object: Box::new(expr),
                        member: part,
                    },
                ));
            }
            expr
        }
    })
    .parse(input)
}

/// Parse a nameof expression: `nameof(identifier)` or `nameof(Class.Member)`
pub fn parse_nameof_expression<'a>(input: Span<'a>) -> BResult<'a, Expression> {
    map(
        preceded(
            kw_nameof(),
            delimited(
                delimited(ws, nom_char('('), ws),
                delimited(ws, parse_qualified_name, ws),
                cut(delimited(ws, nom_char(')'), ws)),
            ),
        ),
        |expr| {
            Expression::Nameof(Box::new(NameofExpression {
                expr: Box::new(expr),
            }))
        },
    )
    .context("nameof expression")
    .parse(input)
}
use crate::syntax::span::Span;
