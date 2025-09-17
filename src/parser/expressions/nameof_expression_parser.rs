use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::nameof_expression::NameofExpression;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};

use nom::combinator::cut;
use nom::{
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
};

/// Parse a qualified name like "MyClass.MyMethod" or just "MyMethod"
fn parse_qualified_name(input: &str) -> BResult<&str, Expression> {
    map(separated_list1(bchar('.'), parse_identifier), |parts| {
        if parts.len() == 1 {
            Expression::Variable(parts.into_iter().next().unwrap())
        } else {
            // Build a chain of member accesses
            let mut expr = Expression::Variable(parts[0].clone());
            for part in parts.into_iter().skip(1) {
                expr = Expression::MemberAccess(Box::new(
                        crate::syntax::nodes::expressions::member_access_expression::MemberAccessExpression {
                            object: Box::new(expr),
                            member: part,
                        }
                    ));
            }
            expr
        }
    })(input)
}

/// Parse a nameof expression: `nameof(identifier)` or `nameof(Class.Member)`
pub fn parse_nameof_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "nameof expression",
        map(
            preceded(
                keyword("nameof"),
                delimited(
                    bws(bchar('(')),
                    bws(parse_qualified_name),
                    cut(bws(bchar(')'))),
                ),
            ),
            |expr| {
                Expression::Nameof(Box::new(NameofExpression {
                    expr: Box::new(expr),
                }))
            },
        ),
    )(input)
}
