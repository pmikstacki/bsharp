use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::assignment_expression::AssignmentExpression;
use crate::syntax::nodes::expressions::conditional_expression::ConditionalExpression;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::BinaryOperator;
use crate::syntax::parser_helpers::{bchar, context, bws, keyword};
use crate::parser::expressions::lambda_expression_parser::parse_lambda_or_anonymous_method;
use crate::parser::expressions::literal_parser::parse_literal;
use crate::parser::expressions::query_expression_parser::parse_query_expression;
use crate::parser::expressions::switch_expression_parser::parse_switch_expression;
// We will handle paren-or-tuple disambiguation locally to avoid early commitment
use crate::parser::expressions::throw_expression_parser::parse_throw_expression;
use crate::parser::expressions::nameof_expression_parser::parse_nameof_expression;
use crate::parser::expressions::default_expression_parser::parse_default_expression;
use crate::parser::expressions::stackalloc_expression_parser::parse_stackalloc_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::expressions::paren_tuple_primary_parser::parse_paren_or_tuple_primary;
use crate::parser::expressions::range_expression_parser::parse_range_expression_or_higher;
use crate::parser::expressions::new_expression_parser::parse_new_expression;

use nom::{
    branch::alt,
    combinator::{map, opt, recognize},
    sequence::{pair, tuple},
};
use crate::syntax::comment_parser::ws;

/// Parse any expression - the main entry point for expression parsing
pub fn parse_expression(input: &str) -> BResult<&str, Expression> {
    context(
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
    let (after_ws, _) = ws(input)?;
    
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

pub(crate) fn parse_primary_expression(input: &str) -> BResult<&str, Expression> {
    context(
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
            // Unified handling for ( ... ) that disambiguates parenthesized vs tuple
            parse_paren_or_tuple_primary,
            // New expressions (includes anonymous object creation)
            parse_new_expression,
            // Lambda expressions
            parse_lambda_or_anonymous_method,
            // Variables/identifiers
            map(parse_identifier, |id| Expression::Variable(id)),
            // Stackalloc expressions
            parse_stackalloc_expression,
        )),
    )(input)
}

