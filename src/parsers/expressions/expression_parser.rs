use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::assignment_expression::AssignmentExpression;
use crate::parser::nodes::expressions::await_expression::AwaitExpression;
use crate::parser::nodes::expressions::conditional_expression::ConditionalExpression;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::expressions::indexing_expression::IndexingExpression;
use crate::parser::nodes::expressions::invocation_expression::InvocationExpression;
use crate::parser::nodes::expressions::literal::Literal;
use crate::parser::nodes::expressions::member_access_expression::MemberAccessExpression;
use crate::parser::nodes::expressions::new_expression::NewExpression;
use crate::parser::nodes::expressions::anonymous_object_creation_expression::{AnonymousObjectCreationExpression, AnonymousObjectMember};
use crate::parser::nodes::expressions::null_conditional_expression::NullConditionalExpression;
use crate::parser::nodes::expressions::range_expression::{IndexExpression, RangeExpression};
use crate::parser::nodes::expressions::BinaryOperator;
use crate::parser::nodes::expressions::UnaryOperator;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword};
use crate::parsers::expressions::lambda_expression_parser::parse_lambda_or_anonymous_method;
use crate::parsers::expressions::literal_parser::parse_literal;
use crate::parsers::expressions::query_expression_parser::parse_query_expression;
use crate::parsers::expressions::switch_expression_parser::parse_switch_expression;
use crate::parsers::expressions::tuple_expression_parser::parse_tuple_expression;
use crate::parsers::expressions::throw_expression_parser::parse_throw_expression;
use crate::parsers::expressions::nameof_expression_parser::parse_nameof_expression;
use crate::parsers::expressions::typeof_expression_parser::parse_typeof_expression;
use crate::parsers::expressions::sizeof_expression_parser::parse_sizeof_expression;
use crate::parsers::expressions::default_expression_parser::parse_default_expression;
use crate::parsers::expressions::stackalloc_expression_parser::parse_stackalloc_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;

use nom::{
    branch::alt,
    combinator::{map, opt, recognize},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    character::complete::multispace0,
};

/// Parse any expression - the main entry point for expression parsing
pub fn parse_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "expression",
        bws(parse_assignment_expression_or_higher),
    )(input)
}

fn parse_assignment_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // Try to parse a conditional expression first
    let (input, left) = parse_conditional_expression_or_higher(input)?;
    
    // Check for assignment operators - order matters, longer operators first
    let (input, assignment_op) = opt(bws(alt((
        // Multi-character assignment operators first
        map(tuple((bchar('?'), bchar('?'), bchar('='))), |_| BinaryOperator::NullCoalescingAssign),
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
    // We need to be careful not to consume ?. or ?[ (null-conditional operators)
    // or ?? (null-coalescing operators)
    
    // First, try to parse whitespace and then ?
    let (after_ws, _) = multispace0(input)?;
    
    // Check if we have a ? that's not followed by . or [ or ?
    if let Ok((after_q, _)) = bchar('?')(after_ws) {
        // Use peek to check what comes next without consuming
        if nom::combinator::peek(nom::combinator::not(alt((bchar('.'), bchar('['), bchar('?')))))(after_q).is_ok() {
            // It's a ternary operator, not null-conditional or null-coalescing
            let (input, _) = bws(bchar('?'))(input)?;
            let (input, true_expr) = bws(parse_expression)(input)?;
            let (input, _) = bws(bchar(':'))(input)?;
            let (input, false_expr) = bws(parse_conditional_expression_or_higher)(input)?;
            
            return Ok((input, Expression::Conditional(Box::new(ConditionalExpression {
                condition: Box::new(condition),
                consequence: Box::new(true_expr),
                alternative: Box::new(false_expr),
            }))));
        }
    }
    
    // Not a ternary operator, just return the condition
    Ok((input, condition))
}

fn parse_null_coalescing_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) = parse_logical_or_expression_or_higher(input)?;
    
    // Handle ?? (null coalescing) - right associative, but avoid consuming if followed by =
    while let Ok((new_input, _)) = bws(tuple((bchar('?'), bchar('?'), nom::combinator::not(bchar('=')))))(input) {
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

// Helper for parse_range_expression_or_higher to specifically parse ranges starting with `..`
fn parse_range_starting_with_dots(input: &str) -> BResult<&str, Expression> {
    let (input, _) = pair(bchar('.'), bchar('.'))(input)?;
    let (input, end_operand) = opt(bws(parse_unary_expression_or_higher))(input)?;
    Ok((
        input,
        Expression::Range(Box::new(RangeExpression {
            start: None,
            end: end_operand.map(Box::new),
            is_inclusive: false,
        })),
    ))
}

// This is the main function that should be in the precedence chain.
// It will first try to parse a range that starts with `..`
// If that fails, it will try to parse a unary expression, and then check if it's followed by `..`
fn parse_range_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    alt((
        // Attempt to parse ranges like `..expr` or `..` first
        bs_context("range starting with ..", parse_range_starting_with_dots),
        // Then, attempt to parse expressions like `expr..`, `expr..expr` or just `expr`
        bs_context("range starting with operand or just operand", |i: &str| {
            let (i, start_expr) = parse_unary_expression_or_higher(i)?;
            if let Ok((i_after_dots, _)) = bws(pair(bchar('.'), bchar('.')))(i) {
                let (i_after_end, end_expr) = opt(parse_unary_expression_or_higher)(i_after_dots)?;
                Ok((i_after_end, Expression::Range(Box::new(RangeExpression {
                    start: Some(Box::new(start_expr)),
                    end: end_expr.map(Box::new),
                    is_inclusive: false,
                }))))
            } else {
                Ok((i, start_expr))
            }
        }),
    ))(input)
}

#[derive(Debug, Clone)]
enum PostfixOpKind { 
    Invocation(Vec<Expression>), 
    MemberAccess(Identifier),
    NullConditionalMemberAccess(Identifier),
    Indexing(Box<Expression>), 
    NullConditionalIndexing(Box<Expression>),
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
        // Add ^ (index from end) operator here. It has high precedence like other unary operators.
        map(bchar('^'), |_| UnaryOperator::IndexFromEnd),
    )))(input) {
        let (input, operand) = parse_unary_expression_or_higher(input)?;
        // If the operator is IndexFromEnd, wrap it in Expression::Index
        if op == UnaryOperator::IndexFromEnd {
            return Ok((input, Expression::Index(Box::new(IndexExpression {
                value: Box::new(operand),
            }))));
        }
        return Ok((input, Expression::Unary {
            op,
            expr: Box::new(operand),
        }));
    }
    
    // Try await expression (reverted to opt + cut version)
    let (input_after_opt_await, opt_await_keyword) = opt(bws(keyword("await")))(input)?;
    if opt_await_keyword.is_some() {
        let (input_after_operand, operand) = parse_unary_expression_or_higher(input_after_opt_await)?;
        return Ok((input_after_operand, Expression::Await(Box::new(AwaitExpression {
            expr: Box::new(operand),
        }))));
    }
    
    // Try cast expression: (Type)expression - but be more careful to avoid conflicts with parenthesized expressions
    if let Ok((input_after_paren, _)) = bws(bchar('('))(input) {
        // Try to parse as a type, but only if it's followed by something that looks like an expression
        if let Ok((input_after_type, _ty)) = parse_type_expression(input_after_paren) {
            if let Ok((input_after_close_paren, _)) = bws(bchar(')'))(input_after_type) {
                // Only treat as cast if there's actually something after the closing parenthesis
                // that could be an expression (not end of input)
                if !input_after_close_paren.trim().is_empty() {
                    let (input, operand) = parse_unary_expression_or_higher(input_after_close_paren)?;
                    return Ok((input, Expression::Unary {
                        op: UnaryOperator::Cast,
                        expr: Box::new(operand),
                    }));
                }
            }
        }
    }
    
    // Try stackalloc expression
    if let Ok((input, stackalloc_expr)) = parse_stackalloc_expression(input) {
        return Ok((input, stackalloc_expr));
    }
    
    // Try sizeof expression
    if let Ok((input, sizeof_expr)) = parse_sizeof_expression(input) {
        return Ok((input, sizeof_expr));
    }
    
    // Try typeof expression
    if let Ok((input, typeof_expr)) = parse_typeof_expression(input) {
        return Ok((input, typeof_expr));
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
            // Null-conditional member access: ?.member
            map(preceded(tuple((bchar('?'), bchar('.'))), bws(parse_identifier)), |id| PostfixOpKind::NullConditionalMemberAccess(id)),
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
            // Null-conditional indexing: ?[index]
            map(
                preceded(
                    bchar('?'),
                    delimited(bchar('['), bws(parse_expression), bws(bchar(']')))
                ),
                |index| PostfixOpKind::NullConditionalIndexing(Box::new(index))
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
                PostfixOpKind::NullConditionalMemberAccess(member) => Expression::NullConditional(Box::new(NullConditionalExpression {
                    target: Box::new(expr),
                    member,
                    is_element_access: false,
                    argument: None,
                })),
                PostfixOpKind::Invocation(args) => Expression::Invocation(Box::new(InvocationExpression {
                    callee: Box::new(expr),
                    arguments: args,
                })),
                PostfixOpKind::Indexing(index) => Expression::Indexing(Box::new(IndexingExpression {
                    target: Box::new(expr),
                    index,
                })),
                PostfixOpKind::NullConditionalIndexing(index) => Expression::NullConditional(Box::new(NullConditionalExpression {
                    target: Box::new(expr),
                    member: Identifier::new(""), // Empty for indexing
                    is_element_access: true,
                    argument: Some(index),
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
            // LINQ Query expressions - must come before variables/identifiers
            parse_query_expression,
            // Switch expressions - must come before variables/identifiers  
            parse_switch_expression,
            // Throw expressions - must come before variables/identifiers
            parse_throw_expression,
            // Nameof expressions - must come before variables/identifiers
            parse_nameof_expression,
            // Default expressions - must come before variables/identifiers
            parse_default_expression,
            // Literals
            map(parse_literal, |lit| Expression::Literal(lit)),
            // this keyword
            map(keyword("this"), |_| Expression::This),
            // Tuple expressions: (expr1, expr2, ...) - must come before general parenthesized expressions
            parse_tuple_expression,
            // Parenthesized expressions - try before new/lambda to avoid potential conflicts with their parameter/arg list parsing
            delimited(bws(bchar('(')), bws(parse_expression), bws(bchar(')'))),
            // New expressions
            parse_new_expression,
            // Lambda expressions
            parse_lambda_or_anonymous_method,
            // Variables/identifiers
            map(parse_identifier, |id| Expression::Variable(id)),
            // Anonymous object creation
            parse_anonymous_object_creation,
            // Stackalloc expressions
            parse_stackalloc_expression,
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
        bws(bchar('{')),
        alt((
            // Object initializer: { prop = value, prop2 = value2 }
            // Must have at least one member assignment.
            map(
                separated_list1(
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
            // Collection initializer: { expr1, expr2, expr3 } or { }
            // Handles non-empty and empty { } cases.
            map(
                separated_list0(bws(bchar(',')), bws(parse_expression)),
                InitializerKind::Collection
            ),
        )),
        bws(bchar('}'))
    )(input)
}

fn parse_anonymous_object_creation(input: &str) -> BResult<&str, Expression> {
    map(
        preceded(
            keyword("new"),
            bs_context(
                "anonymous object creation",
                delimited(
                    bws(bchar('{')),
                    separated_list0(bws(bchar(',')), bws(parse_anonymous_object_member)),
                    bws(bchar('}'))
                ),
            )
        ),
        |members| Expression::AnonymousObject(AnonymousObjectCreationExpression {
            initializers: members,
        })
    )(input)
}

fn parse_anonymous_object_member(input: &str) -> BResult<&str, AnonymousObjectMember> {
    // Handle both explicit (Name = value) and implicit (expression) initializers
    alt((
        // Explicit initializer: Name = value
        map(
            tuple((
                bws(parse_identifier),
                bws(bchar('=')),
                bws(parse_expression),
            )),
            |(name, _, value)| AnonymousObjectMember {
                name: Some(name),
                value,
            }
        ),
        // Implicit initializer: just an expression (for projection)
        map(
            bws(parse_expression),
            |value| AnonymousObjectMember {
                name: None,
                value,
            }
        ),
    ))(input)
}
