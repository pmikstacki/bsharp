use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::expression_keywords::kw_nameof;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;

use nom::Parser;
use nom::character::complete::char as nom_char;
use nom::combinator::cut;
use nom::{
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
};
use nom_supreme::ParserExt;
use syntax::expressions::{Expression, NameofExpression};

/// Parse a qualified name like "MyClass.MyMethod" or just "MyMethod"
fn parse_qualified_name_expr(input: Span) -> BResult<Expression> {
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

//

/// Parse a nameof expression: `nameof(identifier)` or `nameof(Class.Member)` or `nameof(List<>)`/`nameof(Dictionary<,>)`
pub fn parse_nameof_expression(input: Span) -> BResult<Expression> {
    map(
        preceded(
            kw_nameof(),
            delimited(
                delimited(ws, tok_l_paren(), ws),
                |i| {
                    // Parse qualified-name expression first
                    let (i, expr) = delimited(ws, parse_qualified_name_expr, ws).parse(i)?;
                    // Optionally consume an unbound generic marker: <[,]*>
                    let _ = nom::combinator::opt(|j| {
                        let (j, _) = delimited(ws, tok_lt(), ws).parse(j)?;
                        let (j, _) = nom::bytes::complete::take_while(|c: char| c == ',' || c.is_whitespace()).parse(j)?;
                        let (j, _) = cut(delimited(ws, tok_gt(), ws)).parse(j)?;
                        Ok::<_, nom::Err<_>>((j, ()))
                    })
                    .parse(i)?;
                    Ok((i, expr))
                },
                cut(delimited(ws, tok_r_paren(), ws)),
            ),
        ),
        |expr| Expression::Nameof(Box::new(NameofExpression { expr: Box::new(expr) })),
    )
    .context("nameof expression")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
use crate::tokens::relational::{tok_lt, tok_gt};
