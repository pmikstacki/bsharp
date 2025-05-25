use nom::{
    branch::alt,
    character::complete::{multispace0, char as bchar}, 
    combinator::{map, opt},
    multi::{fold_many0, separated_list0},
    sequence::{preceded, tuple, delimited},
};

use crate::parser::errors::{BResult, BSharpParseError};
use crate::parser::nodes::expressions::assignment_expression::AssignmentExpression;
use crate::parser::nodes::expressions::conditional_expression::ConditionalExpression;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::expressions::indexing_expression::IndexingExpression;
use crate::parser::nodes::expressions::invocation_expression::InvocationExpression;
use crate::parser::nodes::expressions::member_access_expression::MemberAccessExpression;
use crate::parser::nodes::expressions::NewExpression;
use crate::parser::nodes::expressions::{BinaryOperator, UnaryOperator};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{keyword, bws};

use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::expressions::literal_parser::parse_literal;
use crate::parsers::types::type_parser::parse_type_expression;

/// Parses a C# expression.
/// This is the main entry point for parsing expressions.
pub fn parse_expression(input: &str) -> BResult<&str, Expression> {
    // Start with the lowest precedence level (assignment or lambda expressions)
    parse_assignment_expression_or_higher(input)
}

// Level 16: Assignment, Compound Assignment, Null-Coalescing Assignment, Lambda
fn parse_assignment_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // TODO: Handle lambdas here as well, as they have the lowest precedence along with assignments.
    // Assignment is right-associative: a = b = c  is a = (b = c)
    // Try parsing a conditional expression first (LHS of a potential assignment)
    let (i, lhs) = parse_conditional_expression_or_higher(input)?;

    // Check for an assignment operator
    let assign_op_parser = alt((
        map(bchar('='), |_| BinaryOperator::Assign),
        map(keyword("+="), |_| BinaryOperator::AddAssign),
        map(keyword("-="), |_| BinaryOperator::SubtractAssign),
        map(keyword("*="), |_| BinaryOperator::MultiplyAssign),
        map(keyword("/="), |_| BinaryOperator::DivideAssign),
        map(keyword("%="), |_| BinaryOperator::ModuloAssign),
        map(keyword("&="), |_| BinaryOperator::AndAssign),
        map(keyword("|="), |_| BinaryOperator::OrAssign),
        map(keyword("^="), |_| BinaryOperator::XorAssign),
        map(keyword("<<="), |_| BinaryOperator::LeftShiftAssign),
        map(keyword(">>="), |_| BinaryOperator::RightShiftAssign),
        // map(keyword(">>>="), |_| BinaryOperator::UnsignedRightShiftAssign), // Need 3-char keyword helper
        map(keyword("??="), |_| BinaryOperator::NullCoalescingAssign),
    ));

    if let Ok((i_after_op, op)) = preceded(multispace0::<&str, BSharpParseError<&str>>, assign_op_parser)(i) {
        // If an assignment operator is found, parse the RHS recursively.
        // The RHS itself can be another assignment expression.
        let (i_final, rhs) = parse_assignment_expression_or_higher(preceded(multispace0::<&str, BSharpParseError<&str>>, |s| Ok((s,())))(i_after_op)?.0)?;
        
        Ok((
            i_final,
            Expression::Assignment(Box::new(AssignmentExpression {
                target: Box::new(lhs),
                op, // This needs to be an AssignmentOperator if your AST distinguishes it for stricter typing
                value: Box::new(rhs),
            })),
        ))
    } else {
        // Not an assignment, return the LHS as is.
        Ok((i, lhs))
    }
}

// Level 15: Conditional (Ternary) Operator `?:` (Right-associative)
fn parse_conditional_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, condition_expr) = parse_null_coalescing_expression_or_higher(input)?;

    if let Ok((i_after_q, _)) = preceded(multispace0::<&str, BSharpParseError<&str>>, bchar('?'))(i) {
        // The 'true' part can be any expression up to assignment (or full expression by C# spec)
        let (i_after_true, true_expr) = parse_assignment_expression_or_higher(preceded(multispace0::<&str, BSharpParseError<&str>>, |s| Ok((s,())))(i_after_q)?.0)?;
        
        if let Ok((i_after_colon, _)) = preceded(multispace0::<&str, BSharpParseError<&str>>, bchar(':'))(i_after_true) {
            // The 'false' part is recursively a conditional_expression_or_higher for right-associativity
            let (i_final, false_expr) = parse_conditional_expression_or_higher(preceded(multispace0::<&str, BSharpParseError<&str>>, |s| Ok((s,())))(i_after_colon)?.0)?;
            
            return Ok((
                i_final,
                Expression::Conditional(Box::new(ConditionalExpression {
                    condition: Box::new(condition_expr),
                    consequence: Box::new(true_expr),
                    alternative: Box::new(false_expr),
                })),
            ));
        }
    }
    // Not a conditional expression, return the condition_expr as is.
    Ok((i, condition_expr))
}

// Level 14: Null Coalescing Operator `??` (Right-associative)
fn parse_null_coalescing_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, lhs) = parse_logical_or_expression_or_higher(input)?;

    if let Ok((i_after_op, _)) = preceded(multispace0::<&str, BSharpParseError<&str>>, keyword("??"))(i) {
        // Parse RHS recursively for right-associativity
        let (i_final, rhs) = parse_null_coalescing_expression_or_higher(preceded(multispace0::<&str, BSharpParseError<&str>>, |s| Ok((s,())))(i_after_op)?.0)?;
        Ok((
            i_final,
            Expression::Binary {
                left: Box::new(lhs),
                op: BinaryOperator::NullCoalescing,
                right: Box::new(rhs),
            }
        ))
    } else {
        Ok((i, lhs))
    }
}

// Level 13: Logical OR `||`
fn parse_logical_or_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_logical_and_expression_or_higher(input)?;
    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            map(keyword("||"), |_| BinaryOperator::LogicalOr),
            multispace0::<&str, BSharpParseError<&str>>,
            parse_logical_and_expression_or_higher
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 12: Logical AND `&&`
fn parse_logical_and_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_bitwise_or_expression_or_higher(input)?;
    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            map(keyword("&&"), |_| BinaryOperator::LogicalAnd),
            multispace0::<&str, BSharpParseError<&str>>,
            parse_bitwise_or_expression_or_higher
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 11: Bitwise OR `|`
fn parse_bitwise_or_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_bitwise_xor_expression_or_higher(input)?;
    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            map(bchar('|'), |_| BinaryOperator::BitwiseOr), 
            multispace0::<&str, BSharpParseError<&str>>,
            parse_bitwise_xor_expression_or_higher
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 10: Bitwise XOR `^`
fn parse_bitwise_xor_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_bitwise_and_expression_or_higher(input)?;
    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            map(bchar('^'), |_| BinaryOperator::BitwiseXor), 
            multispace0::<&str, BSharpParseError<&str>>,
            parse_bitwise_and_expression_or_higher
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 9: Bitwise AND `&`
fn parse_bitwise_and_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_equality_expression_or_higher(input)?;
    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            map(bchar('&'), |_| BinaryOperator::BitwiseAnd), 
            multispace0::<&str, BSharpParseError<&str>>,
            parse_equality_expression_or_higher
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 8: Equality `==`, `!=`
fn parse_equality_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_relational_expression_or_higher(input)?;
    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            alt((
                map(keyword("=="), |_| BinaryOperator::Equal),
                map(keyword("!="), |_| BinaryOperator::NotEqual),
            )),
            multispace0::<&str, BSharpParseError<&str>>,
            parse_relational_expression_or_higher
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 7: Relational `<`, `>`, `<=`, `>=`, `is`, `as`
fn parse_relational_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_shift_expression_or_higher(input)?;
    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            alt((
                map(keyword("<="), |_| BinaryOperator::LessEqual),
                map(keyword(">="), |_| BinaryOperator::GreaterEqual),
                map(keyword("<"), |_| BinaryOperator::LessThan), 
                map(keyword(">"), |_| BinaryOperator::GreaterThan), 
                map(keyword("is"), |_| BinaryOperator::Is), 
                map(keyword("as"), |_| BinaryOperator::As), 
            )),
            multispace0::<&str, BSharpParseError<&str>>,
            parse_shift_expression_or_higher 
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 6: Shift `<<`, `>>`, `>>>`
fn parse_shift_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_additive_expression_or_higher(input)?;
    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            alt((
                map(keyword("<<"), |_| BinaryOperator::LeftShift),
                map(keyword(">>"), |_| BinaryOperator::RightShift),
            )),
            multispace0::<&str, BSharpParseError<&str>>,
            parse_additive_expression_or_higher
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 5: Additive `+`, `-` (Original parse_additive_expression will be adapted)
fn parse_additive_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_multiplicative_expression_or_higher(input)?;

    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            alt((
                map(bchar('+'), |_| BinaryOperator::Add),
                map(bchar('-'), |_| BinaryOperator::Subtract),
            )),
            multispace0::<&str, BSharpParseError<&str>>,
            parse_multiplicative_expression_or_higher
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 4: Multiplicative `*`, `/`, `%` (Original parse_multiplicative_expression will be adapted)
fn parse_multiplicative_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_range_expression_or_higher(input)?;

    fold_many0(
        tuple((
            multispace0::<&str, BSharpParseError<&str>>,
            alt((
                map(bchar('*'), |_| BinaryOperator::Multiply),
                map(bchar('/'), |_| BinaryOperator::Divide),
                map(bchar('%'), |_| BinaryOperator::Modulo),
            )),
            multispace0::<&str, BSharpParseError<&str>>,
            parse_range_expression_or_higher 
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| Expression::Binary { left: Box::new(acc), op, right: Box::new(right) }
    )(i)
}

// Level 3: Range `..` (New level)
fn parse_range_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (i, initial) = parse_unary_expression_or_higher(input)?;

    let res_opt = opt(tuple((
        multispace0::<&str, BSharpParseError<&str>>,
        map(keyword(".."), |_| BinaryOperator::Range),
        multispace0::<&str, BSharpParseError<&str>>,
        parse_unary_expression_or_higher 
    )))(i);

    match res_opt {
        Ok((i_final, Some((_, op, _, right_expr)))) => {
            Ok((i_final, Expression::Binary {
                left: Box::new(initial),
                op,
                right: Box::new(right_expr),
            }))
        }
        _ => Ok((i, initial)), 
    }
}

// Level 2: Unary `+ - ! ~ ++ -- (T) await ^ & *` (New level)
#[derive(Debug, Clone)]
enum PostfixOpKind { 
    Invocation(Vec<Expression>), 
    MemberAccess(Identifier),
    Indexing(Box<Expression>), 
    PostfixIncrement,
    PostfixDecrement,
    NullForgiving,
}

fn parse_unary_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    alt((
        map(
            tuple((keyword("++"), multispace0::<&str, BSharpParseError<&str>>, parse_unary_expression_or_higher)),
            |(_, _, expr)| Expression::Unary {
                op: UnaryOperator::Increment,
                expr: Box::new(expr),
            }
        ),
        map(
            tuple((keyword("--"), multispace0::<&str, BSharpParseError<&str>>, parse_unary_expression_or_higher)),
            |(_, _, expr)| Expression::Unary {
                op: UnaryOperator::Decrement,
                expr: Box::new(expr),
            }
        ),
        map(
            tuple((bchar('+'), multispace0::<&str, BSharpParseError<&str>>, parse_unary_expression_or_higher)),
            |(_, _, expr)| Expression::Unary {
                op: UnaryOperator::Plus,
                expr: Box::new(expr),
            }
        ),
        map(
            tuple((bchar('-'), multispace0::<&str, BSharpParseError<&str>>, parse_unary_expression_or_higher)),
            |(_, _, expr)| Expression::Unary {
                op: UnaryOperator::Minus,
                expr: Box::new(expr),
            }
        ),
        map(
            tuple((bchar('!'), multispace0::<&str, BSharpParseError<&str>>, parse_unary_expression_or_higher)), 
            |(_, _, expr)| Expression::Unary {
                op: UnaryOperator::LogicalNot,
                expr: Box::new(expr),
            }
        ),
        map(
            tuple((bchar('~'), multispace0::<&str, BSharpParseError<&str>>, parse_unary_expression_or_higher)),
            |(_, _, expr)| Expression::Unary {
                op: UnaryOperator::BitwiseNot,
                expr: Box::new(expr),
            }
        ),
        map(
            tuple((bchar('^'), multispace0::<&str, BSharpParseError<&str>>, parse_unary_expression_or_higher)),
            |(_, _, expr)| Expression::Unary {
                op: UnaryOperator::IndexFromEnd,
                expr: Box::new(expr),
            }
        ),
        map(
            tuple((keyword("await"), multispace0::<&str, BSharpParseError<&str>>, parse_unary_expression_or_higher)),
            |(_, _, expr)| Expression::Unary { 
                op: UnaryOperator::Await,
                expr: Box::new(expr),
            }
        ),
        parse_postfix_expression_or_higher,
    ))(input)
}

// Level 1: Postfix (member access, invocation, indexing, postfix `++ -- !`)
fn parse_postfix_expression_or_higher(input: &str) -> BResult<&str, Expression> { 
    let (i, initial_expr): (&str, Expression) = parse_primary_expression(input)?;

    fold_many0(
        alt::<_, _, BSharpParseError<&str>, _>((
            map(
                tuple((
                    bws(bchar('(')),
                    separated_list0(bws(bchar(',')), parse_expression),
                    bws(bchar(')')),
                )),
                |(_, args, _): (_, Vec<Expression>, _)| PostfixOpKind::Invocation(args)
            ),
            map(
                preceded(bchar('.'), preceded(multispace0::<&str, BSharpParseError<&str>>, parse_identifier)),
                |name: Identifier| PostfixOpKind::MemberAccess(name)
            ),
            map(
                tuple((bws(bchar('[')), parse_expression, bws(bchar(']')))),
                |(_, index_val, _): (_, Expression, _)| PostfixOpKind::Indexing(Box::new(index_val))
            ),
            map(preceded(multispace0::<&str, BSharpParseError<&str>>, keyword("++")),
                |_| PostfixOpKind::PostfixIncrement
            ),
            map(preceded(multispace0::<&str, BSharpParseError<&str>>, keyword("--")),
                |_| PostfixOpKind::PostfixDecrement
            ),
            map(preceded(multispace0::<&str, BSharpParseError<&str>>, bchar('!')),
                |_| PostfixOpKind::NullForgiving
            ),
        )),
        move || initial_expr.clone(), 
        |acc_expr: Expression, op_kind: PostfixOpKind| -> Expression { 
            match op_kind {
                PostfixOpKind::Invocation(args) => Expression::Invocation(Box::new(InvocationExpression {
                    callee: Box::new(acc_expr),
                    arguments: args,
                })),
                PostfixOpKind::MemberAccess(name) => Expression::MemberAccess(Box::new(MemberAccessExpression {
                    object: Box::new(acc_expr),
                    member: name, 
                })),
                PostfixOpKind::Indexing(index_expr) => Expression::Indexing(Box::new(IndexingExpression {
                    target: Box::new(acc_expr),
                    index: index_expr,
                })),
                PostfixOpKind::PostfixIncrement => Expression::PostfixUnary {
                    op: UnaryOperator::Increment,
                    expr: Box::new(acc_expr),
                },
                PostfixOpKind::PostfixDecrement => Expression::PostfixUnary {
                    op: UnaryOperator::Decrement,
                    expr: Box::new(acc_expr),
                },
                PostfixOpKind::NullForgiving => Expression::PostfixUnary {
                    op: UnaryOperator::NullForgiving,
                    expr: Box::new(acc_expr),
                },
            }
        }
    )(i)
}

// Level 0: Primary (literals, identifiers, parenthesized expressions, `new`, `typeof`, `default`)
fn parse_primary_expression(input: &str) -> BResult<&str, Expression> {
    bws(alt((
        map(parse_literal, Expression::Literal),
        map(parse_identifier, Expression::Variable),
        // Parenthesized expression: ( expression )
        delimited(bchar('('), parse_expression, bchar(')')),
        // New expression: new Type(...)
        map(
            tuple((
                keyword("new"),
                parse_type_expression,
                bws(delimited(
                    bchar('('),
                    separated_list0(bws(bchar(',')), parse_expression),
                    bchar(')')
                ))
            )),
            |(_, ty, arguments)| Expression::New(Box::new(NewExpression {
                ty,
                arguments,
                object_initializer: None, // TODO: Implement object initializers
                collection_initializer: None, // TODO: Implement collection initializers
            }))
        ),
        // `this` keyword
        map(keyword("this"), |_| Expression::This),
        // `base` keyword - often for calling base class constructors or methods
        map(keyword("base"), |_| Expression::Base),
        // TODO: typeof, default, etc.
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::nodes::expressions::literal::Literal;
    use crate::parser::nodes::types::Type;

    #[test]
    fn test_parse_simple_new_expression() {
        let input = "new Exception(\"Error\")";
        let result = parse_expression(input);
        assert!(result.is_ok(), "Failed to parse 'new Exception(\"Error\")': {:?}", result.err());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        match expr {
            Expression::New(boxed_new_expr) => {
                match &boxed_new_expr.ty {
                    Type::Reference(ident) => assert_eq!(ident.name, "Exception"),
                    _ => panic!("Expected Type::Reference for new expression type"),
                }
                assert_eq!(boxed_new_expr.arguments.len(), 1);
                match &boxed_new_expr.arguments[0] {
                    Expression::Literal(Literal::String(s)) => assert_eq!(s, "Error"),
                    _ => panic!("Expected string literal argument"),
                }
                assert!(boxed_new_expr.object_initializer.is_none());
                assert!(boxed_new_expr.collection_initializer.is_none());
            }
            _ => panic!("Expected Expression::New, got {:?}", expr),
        }
    }

    #[test]
    fn test_parse_new_expression_no_args() {
        let input = "new Object()";
        let result = parse_expression(input);
        assert!(result.is_ok(), "Failed to parse 'new Object()': {:?}", result.err());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        match expr {
            Expression::New(boxed_new_expr) => {
                match &boxed_new_expr.ty {
                    Type::Reference(ident) => assert_eq!(ident.name, "Object"),
                    _ => panic!("Expected Type::Reference for new expression type"),
                }
                assert!(boxed_new_expr.arguments.is_empty());
            }
            _ => panic!("Expected Expression::New, got {:?}", expr),
        }
    }

    #[test]
    fn test_parse_new_expression_multiple_args() {
        let input = "new Data(42, \"test\", true)";
        let result = parse_expression(input);
        assert!(result.is_ok(), "Failed to parse 'new Data(42, \"test\", true)': {:?}", result.err());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        match expr {
            Expression::New(boxed_new_expr) => {
                match &boxed_new_expr.ty {
                    Type::Reference(ident) => assert_eq!(ident.name, "Data"),
                    _ => panic!("Expected Type::Reference for new expression type"),
                }
                assert_eq!(boxed_new_expr.arguments.len(), 3);
                match &boxed_new_expr.arguments[0] {
                    Expression::Literal(Literal::Integer(i)) => assert_eq!(*i, 42),
                    _ => panic!("Expected integer literal for first argument"),
                }
                match &boxed_new_expr.arguments[1] {
                    Expression::Literal(Literal::String(s)) => assert_eq!(s, "test"),
                    _ => panic!("Expected string literal for second argument"),
                }
                match &boxed_new_expr.arguments[2] {
                    Expression::Literal(Literal::Boolean(b)) => assert_eq!(*b, true),
                    _ => panic!("Expected boolean literal for third argument"),
                }
            }
            _ => panic!("Expected Expression::New, got {:?}", expr),
        }
    }

    #[test]
    fn test_basic_identifier() {
        // ... tests ...
    }
}
