use crate::syntax::errors::BResult;
use crate::syntax::span::Span;

use nom::combinator::cut;
use nom::Err;
use nom::Parser;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
use syntax::expressions::{BinaryOperator, Expression};

/// Generic left-associative chain builder.
///
/// - `next` parses the higher-precedence expression on both sides of the operator.
/// - `op` parses exactly one operator occurrence and returns a `BinaryOperator`.
///
/// left_chain(next, op) parses: next (op next)* and folds left-associatively.
pub fn left_chain<'a, FNext, FOp>(
    mut next: FNext,
    mut op: FOp,
) -> impl FnMut(Span<'a>) -> BResult<'a, Expression>
where
    FNext: FnMut(Span<'a>) -> BResult<'a, Expression>,
    FOp: FnMut(Span<'a>) -> BResult<'a, BinaryOperator>,
{
    move |mut input: Span<'a>| {
        // parse first operand
        let (mut i, mut left) = next(input.into())?;
        // loop for (op next)*
        loop {
            match op(i) {
                Ok((i_after_op, bop)) => match cut(|j| next(j)).parse(i_after_op) {
                    Ok((i_after_rhs, right)) => {
                        left = Expression::Binary {
                            left: Box::new(left),
                            op: bop,
                            right: Box::new(right),
                        };
                        i = i_after_rhs;
                    }
                    Err(Err::Error(_)) | Err(Err::Failure(_)) => {
                        let expectation = bop.rhs_expectation();
                        let err = ErrorTree::Base {
                            location: i_after_op,
                            kind: BaseErrorKind::Expected(Expectation::Tag(expectation)),
                        };
                        return Err(Err::Failure(err));
                    }
                    Err(Err::Incomplete(needed)) => {
                        return Err(Err::Incomplete(needed));
                    }
                },
                Err(_) => {
                    input = i;
                    break;
                }
            }
        }
        Ok((input, left))
    }
}
