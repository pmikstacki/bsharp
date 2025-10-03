use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::{BinaryOperator, Expression};
use nom::Err;
use nom::combinator::cut;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};

/// Generic left-associative chain builder.
///
/// - `next` parses the higher-precedence expression on both sides of the operator.
/// - `op` parses exactly one operator occurrence and returns a `BinaryOperator`.
///
/// left_chain(next, op) parses: next (op next)* and folds left-associatively.
pub fn left_chain<'a, FNext, FOp>(
    mut next: FNext,
    mut op: FOp,
) -> impl FnMut(&'a str) -> BResult<&'a str, Expression>
where
    FNext: FnMut(&'a str) -> BResult<&'a str, Expression>,
    FOp: FnMut(&'a str) -> BResult<&'a str, BinaryOperator>,
{
    move |mut input: &'a str| {
        // parse first operand
        let (mut i, mut left) = next(input)?;
        // loop for (op next)*
        loop {
            match op(i) {
                Ok((i_after_op, bop)) => match cut(&mut next)(i_after_op) {
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
