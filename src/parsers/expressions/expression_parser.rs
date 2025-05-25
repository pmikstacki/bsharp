use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::expressions::literal::Literal;
use crate::parser::nodes::expressions::new_expression::NewExpression;
use crate::parser::nodes::expressions::assignment_expression::AssignmentExpression;
use crate::parser::nodes::expressions::conditional_expression::ConditionalExpression;
use crate::parser::nodes::expressions::member_access_expression::MemberAccessExpression;
use crate::parser::nodes::expressions::invocation_expression::InvocationExpression;
use crate::parser::nodes::expressions::indexing_expression::IndexingExpression;
use crate::parser::nodes::expressions::BinaryOperator;
use crate::parser::nodes::expressions::UnaryOperator;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword};
use crate::parsers::expressions::literal_parser::parse_literal;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;

use nom::{
    combinator::{map, opt, recognize},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, tuple},
    branch::alt,
};

/// Parse any expression - the main entry point for expression parsing
pub fn parse_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "expression",
        parse_assignment_expression_or_higher,
    )(input)
}

fn parse_assignment_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // Try to parse a conditional expression first
    let (input, left) = parse_conditional_expression_or_higher(input)?;
    
    // Check for assignment operators - order matters, longer operators first
    let (input, assignment_op) = opt(bws(alt((
        // Multi-character assignment operators first
        map(tuple((bchar('<'), bchar('<'), bchar('='))), |_| BinaryOperator::LeftShiftAssign),
        map(tuple((bchar('>'), bchar('>'), bchar('='))), |_| BinaryOperator::RightShiftAssign),
        map(tuple((bchar('+'), bchar('='))), |_| BinaryOperator::AddAssign),
        map(tuple((bchar('-'), bchar('='))), |_| BinaryOperator::SubtractAssign),
        map(tuple((bchar('*'), bchar('='))), |_| BinaryOperator::MultiplyAssign),
        map(tuple((bchar('/'), bchar('='))), |_| BinaryOperator::DivideAssign),
        map(tuple((bchar('%'), bchar('='))), |_| BinaryOperator::ModuloAssign),
        map(tuple((bchar('&'), bchar('='))), |_| BinaryOperator::AndAssign),
        map(tuple((bchar('|'), bchar('='))), |_| BinaryOperator::OrAssign),
        map(tuple((bchar('^'), bchar('='))), |_| BinaryOperator::XorAssign),
        // Simple assignment last
        map(bchar('='), |_| BinaryOperator::Assign),
    ))))(input)?;
    
    if let Some(op) = assignment_op {
        // Parse the right side of the assignment (right-associative)
        let (input, right) = parse_assignment_expression_or_higher(input)?;
        
        Ok((input, Expression::Assignment(Box::new(AssignmentExpression {
            target: Box::new(left),
            op,
            value: Box::new(right),
        }))))
    } else {
        Ok((input, left))
    }
}

fn parse_conditional_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // Parse the condition (left side)
    let (input, condition) = parse_null_coalescing_expression_or_higher(input)?;
    
    // Check for ternary operator: condition ? true_expr : false_expr
    let (input, ternary_result) = opt(tuple((
        bws(bchar('?')),
        bws(parse_expression), // true expression
        bws(bchar(':')),
        bws(parse_conditional_expression_or_higher), // false expression (right-associative)
    )))(input)?;
    
    if let Some((_, true_expr, _, false_expr)) = ternary_result {
        Ok((input, Expression::Conditional(Box::new(ConditionalExpression {
            condition: Box::new(condition),
            consequence: Box::new(true_expr),
            alternative: Box::new(false_expr),
        }))))
    } else {
        Ok((input, condition))
    }
}

fn parse_null_coalescing_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_logical_or_expression_or_higher(input)?;
    
    // Handle ?? (null coalescing) - right associative
    while let Ok((new_input, _)) = bws(recognize(pair(bchar('?'), bchar('?'))))(input) {
        let (new_input, right) = parse_null_coalescing_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op: BinaryOperator::NullCoalescing,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_logical_or_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_logical_and_expression_or_higher(input)?;
    
    // Handle || (logical OR) - left associative
    while let Ok((new_input, _)) = bws(recognize(pair(bchar('|'), bchar('|'))))(input) {
        let (new_input, right) = parse_logical_and_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op: BinaryOperator::LogicalOr,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_logical_and_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_bitwise_or_expression_or_higher(input)?;
    
    // Handle && (logical AND) - left associative
    while let Ok((new_input, _)) = bws(recognize(pair(bchar('&'), bchar('&'))))(input) {
        let (new_input, right) = parse_bitwise_or_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op: BinaryOperator::LogicalAnd,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_bitwise_or_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_bitwise_xor_expression_or_higher(input)?;
    
    // Handle | (bitwise OR) - left associative, but avoid consuming if followed by = or |
    while let Ok((new_input, _)) = bws(tuple((bchar('|'), nom::combinator::not(alt((bchar('='), bchar('|')))))))(input) {
        let (new_input, right) = parse_bitwise_xor_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op: BinaryOperator::BitwiseOr,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_bitwise_xor_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_bitwise_and_expression_or_higher(input)?;
    
    // Handle ^ (bitwise XOR) - left associative, but avoid consuming if followed by =
    while let Ok((new_input, _)) = bws(tuple((bchar('^'), nom::combinator::not(bchar('=')))))(input) {
        let (new_input, right) = parse_bitwise_and_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op: BinaryOperator::BitwiseXor,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_bitwise_and_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_equality_expression_or_higher(input)?;
    
    // Handle & (bitwise AND) - left associative, but avoid consuming if followed by = or &
    while let Ok((new_input, _)) = bws(tuple((bchar('&'), nom::combinator::not(alt((bchar('='), bchar('&')))))))(input) {
        let (new_input, right) = parse_equality_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op: BinaryOperator::BitwiseAnd,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_equality_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_relational_expression_or_higher(input)?;
    
    // Handle == and != - left associative
    while let Ok((new_input, op)) = bws(alt((
        map(recognize(pair(bchar('='), bchar('='))), |_| BinaryOperator::Equal),
        map(recognize(pair(bchar('!'), bchar('='))), |_| BinaryOperator::NotEqual),
    )))(input) {
        let (new_input, right) = parse_relational_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_relational_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_shift_expression_or_higher(input)?;
    
    // Handle <, >, <=, >= - left associative
    while let Ok((new_input, op)) = bws(alt((
        map(recognize(pair(bchar('<'), bchar('='))), |_| BinaryOperator::LessEqual),
        map(recognize(pair(bchar('>'), bchar('='))), |_| BinaryOperator::GreaterEqual),
        map(bchar('<'), |_| BinaryOperator::LessThan),
        map(bchar('>'), |_| BinaryOperator::GreaterThan),
    )))(input) {
        let (new_input, right) = parse_shift_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_shift_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_additive_expression_or_higher(input)?;
    
    // Handle << and >> - left associative, but avoid consuming if followed by =
    while let Ok((new_input, op)) = bws(alt((
        map(tuple((bchar('<'), bchar('<'), nom::combinator::not(bchar('=')))), |_| BinaryOperator::LeftShift),
        map(tuple((bchar('>'), bchar('>'), nom::combinator::not(bchar('=')))), |_| BinaryOperator::RightShift),
    )))(input) {
        let (new_input, right) = parse_additive_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_additive_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_multiplicative_expression_or_higher(input)?;
    
    // Handle + and - - left associative, but avoid consuming if followed by =
    while let Ok((new_input, op)) = bws(alt((
        map(tuple((bchar('+'), nom::combinator::not(bchar('=')))), |_| BinaryOperator::Add),
        map(tuple((bchar('-'), nom::combinator::not(bchar('=')))), |_| BinaryOperator::Subtract),
    )))(input) {
        let (new_input, right) = parse_multiplicative_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_multiplicative_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_range_expression_or_higher(input)?;
    
    // Handle *, /, % - left associative, but avoid consuming if followed by =
    while let Ok((new_input, op)) = bws(alt((
        map(tuple((bchar('*'), nom::combinator::not(bchar('=')))), |_| BinaryOperator::Multiply),
        map(tuple((bchar('/'), nom::combinator::not(bchar('=')))), |_| BinaryOperator::Divide),
        map(tuple((bchar('%'), nom::combinator::not(bchar('=')))), |_| BinaryOperator::Modulo),
    )))(input) {
        let (new_input, right) = parse_range_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
        input = new_input;
    }
    
    Ok((input, left))
}

fn parse_range_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // For now, just pass through to unary
    // TODO: Implement range expressions (.., ..^, ^..)
    parse_unary_expression_or_higher(input)
}

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
    // Try prefix unary operators first
    if let Ok((input, op)) = bws(alt((
        map(bchar('+'), |_| UnaryOperator::Plus),
        map(bchar('-'), |_| UnaryOperator::Minus),
        map(bchar('!'), |_| UnaryOperator::LogicalNot),
        map(bchar('~'), |_| UnaryOperator::BitwiseNot),
        map(recognize(pair(bchar('+'), bchar('+'))), |_| UnaryOperator::Increment),
        map(recognize(pair(bchar('-'), bchar('-'))), |_| UnaryOperator::Decrement),
        map(bchar('&'), |_| UnaryOperator::AddressOf),
        map(bchar('*'), |_| UnaryOperator::PointerIndirection),
    )))(input) {
        let (input, operand) = parse_unary_expression_or_higher(input)?;
        return Ok((input, Expression::Unary {
            op,
            expr: Box::new(operand),
        }));
    }
    
    // Try await expression
    if let Ok((input, _)) = bws(keyword("await"))(input) {
        let (input, operand) = parse_unary_expression_or_higher(input)?;
        return Ok((input, Expression::Unary {
            op: UnaryOperator::Await,
            expr: Box::new(operand),
        }));
    }
    
    // Try cast expression: (Type)expression
    if let Ok((input, _)) = bws(bchar('('))(input) {
        // Try to parse as a type
        if let Ok((input, _ty)) = parse_type_expression(input) {
            if let Ok((input, _)) = bws(bchar(')'))(input) {
                let (input, operand) = parse_unary_expression_or_higher(input)?;
                return Ok((input, Expression::Unary {
                    op: UnaryOperator::Cast,
                    expr: Box::new(operand),
                }));
            }
        }
    }
    
    // Try sizeof expression - for now, treat as unary operator
    if let Ok((input, _)) = bws(keyword("sizeof"))(input) {
        let (input, _) = bws(bchar('('))(input)?;
        let (input, _ty) = bws(parse_type_expression)(input)?;
        let (input, _) = bws(bchar(')'))(input)?;
        // For now, create a dummy expression since we don't have a proper sizeof AST node
        return Ok((input, Expression::Literal(Literal::Integer(0))));
    }
    
    // Try typeof expression - for now, treat as unary operator  
    if let Ok((input, _)) = bws(keyword("typeof"))(input) {
        let (input, _) = bws(bchar('('))(input)?;
        let (input, _ty) = bws(parse_type_expression)(input)?;
        let (input, _) = bws(bchar(')'))(input)?;
        // For now, create a dummy expression since we don't have a proper typeof AST node
        return Ok((input, Expression::Literal(Literal::Integer(0))));
    }
    
    // If none of the above work, try postfix expressions
    parse_postfix_expression_or_higher(input)
}

fn parse_postfix_expression_or_higher(input: &str) -> BResult<&str, Expression> { 
    // Start with a primary expression
    let (mut input, mut expr) = parse_primary_expression(input)?;
    
    // Parse zero or more postfix operations
    loop {
        // Try to parse various postfix operations
        if let Ok((new_input, op)) = bws(alt((
            // Member access: .member
            map(preceded(bchar('.'), bws(parse_identifier)), |id| PostfixOpKind::MemberAccess(id)),
            // Method call: (args...)
            map(
                delimited(
                    bchar('('),
                    separated_list0(bws(bchar(',')), bws(parse_expression)),
                    bws(bchar(')'))
                ),
                |args| PostfixOpKind::Invocation(args)
            ),
            // Array indexing: [index]
            map(
                delimited(bchar('['), bws(parse_expression), bws(bchar(']'))),
                |index| PostfixOpKind::Indexing(Box::new(index))
            ),
            // Postfix increment: ++
            map(recognize(pair(bchar('+'), bchar('+'))), |_| PostfixOpKind::PostfixIncrement),
            // Postfix decrement: --
            map(recognize(pair(bchar('-'), bchar('-'))), |_| PostfixOpKind::PostfixDecrement),
            // Null-forgiving: !
            map(bchar('!'), |_| PostfixOpKind::NullForgiving),
        )))(input) {
            // Apply the postfix operation to the current expression
            expr = match op {
                PostfixOpKind::MemberAccess(member) => Expression::MemberAccess(Box::new(MemberAccessExpression {
                    object: Box::new(expr),
                    member,
                })),
                PostfixOpKind::Invocation(args) => Expression::Invocation(Box::new(InvocationExpression {
                    callee: Box::new(expr),
                    arguments: args,
                })),
                PostfixOpKind::Indexing(index) => Expression::Indexing(Box::new(IndexingExpression {
                    target: Box::new(expr),
                    index,
                })),
                PostfixOpKind::PostfixIncrement => Expression::PostfixUnary {
                    op: UnaryOperator::Increment,
                    expr: Box::new(expr),
                },
                PostfixOpKind::PostfixDecrement => Expression::PostfixUnary {
                    op: UnaryOperator::Decrement,
                    expr: Box::new(expr),
                },
                PostfixOpKind::NullForgiving => Expression::PostfixUnary {
                    op: UnaryOperator::NullForgiving,
                    expr: Box::new(expr),
                },
            };
            input = new_input;
        } else {
            // No more postfix operations
            break;
        }
    }
    
    Ok((input, expr))
}

fn parse_primary_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "primary expression",
        alt((
            // Literals
            map(parse_literal, |lit| Expression::Literal(lit)),
            // this keyword
            map(keyword("this"), |_| Expression::This),
            // New expressions
            parse_new_expression,
            // Parenthesized expressions - for now, just unwrap the inner expression
            delimited(bws(bchar('(')), bws(parse_expression), bws(bchar(')'))),
            // Variables/identifiers
            map(parse_identifier, |id| Expression::Variable(id)),
        )),
    )(input)
}

fn parse_new_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "new expression",
        map(
            tuple((
                keyword("new"),
                bws(parse_type_expression),
                opt(delimited(
                    bws(bchar('(')),
                    separated_list0(bws(bchar(',')), bws(parse_expression)),
                    bws(bchar(')'))
                )),
                opt(bws(parse_initializer)),
            )),
            |(_new_kw, ty, arguments, initializer)| {
                let (object_initializer, collection_initializer) = match initializer {
                    Some(InitializerKind::Object(obj)) => (Some(obj), None),
                    Some(InitializerKind::Collection(coll)) => (None, Some(coll)),
                    None => (None, None),
                };
                
                Expression::New(Box::new(NewExpression {
                    ty,
                    arguments: arguments.unwrap_or_default(),
                    object_initializer,
                    collection_initializer,
                }))
            },
        ),
    )(input)
}

#[derive(Debug, Clone)]
enum InitializerKind {
    Object(Vec<(String, Expression)>),
    Collection(Vec<Expression>),
}

fn parse_initializer(input: &str) -> BResult<&str, InitializerKind> {
    delimited(
        bchar('{'),
        alt((
            // Try object initializer first: { prop = value, prop2 = value2 }
            map(
                separated_list0(
                    bws(bchar(',')),
                    tuple((
                        bws(parse_identifier),
                        bws(bchar('=')),
                        bws(parse_expression),
                    ))
                ),
                |pairs| InitializerKind::Object(
                    pairs.into_iter().map(|(id, _, expr)| (id.name, expr)).collect()
                )
            ),
            // Collection initializer: { expr1, expr2, expr3 }
            map(
                separated_list0(bws(bchar(',')), bws(parse_expression)),
                InitializerKind::Collection
            )
        )),
        bchar('}')
    )(input)
}
