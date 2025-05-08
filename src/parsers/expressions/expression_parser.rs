use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::expressions::assignment_expression::AssignmentExpression;
use crate::parser::nodes::expressions::invocation_expression::InvocationExpression;
use crate::parser::nodes::expressions::member_access_expression::MemberAccessExpression;
use crate::parser::nodes::expressions::BinaryOperator;
use nom::{
    branch::alt,
    combinator::map,
    sequence::{delimited, tuple},
    character::complete::multispace0,
    multi::{separated_list0, fold_many0},
};
use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::parser_helpers::{bchar, keyword, bs_context, bws, nom_to_bs}; 
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::expressions::literal_parser::{parse_boolean, parse_char_literal, parse_integer, parse_string}; 

// The main expression parser - starts with the lowest precedence (assignment)
pub fn parse_expression(input: &str) -> BResult<&str, Expression> {
    bs_context("expression", parse_assignment_expression_or_higher)(input)
}

// Tries to parse an assignment; if not, proceeds to higher precedence expressions.
fn parse_assignment_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    alt((
        bs_context(
            "assignment expression",
            map(
                tuple((
                    // LHS: Parsed as a postfix expression to allow identifiers, member access, etc.
                    parse_postfix_expression, 
                    multispace0,
                    bchar('='),
                    multispace0,
                    // RHS: Can be any expression, so we call the main parse_expression recursively.
                    // This handles cases like x = y = z or x = (a + b).
                    parse_expression 
                )),
                |(lhs, _, _, _, rhs)| {
                    // Note: Semantic validation (e.g., ensuring lhs is a valid l-value)
                    // would typically occur in a later compiler stage (e.g., semantic analysis).
                    // Here, we structurally create the assignment.
                    Expression::Assignment(Box::new(AssignmentExpression {
                        target: Box::new(lhs),
                        op: crate::parser::nodes::expressions::BinaryOperator::Assign,
                        value: Box::new(rhs),
                    }))
                }
            )
        ),
        // If it's not an assignment, parse it as an additive expression (next level of precedence)
        parse_additive_expression
    ))(input)
}

// Handle postfix expressions (member access, method invocation)
fn parse_postfix_expression(input: &str) -> BResult<&str, Expression> {
    // Start with a primary expression
    let (mut i, mut expr) = parse_primary_expression(input)?;
    
    // Keep applying postfix operations as long as they match
    loop {
        // Try to parse a method invocation
        match delimited(
            multispace0,
            delimited(
                bchar('('),
                separated_list0(delimited(multispace0, bchar(','), multispace0), parse_expression),
                bchar(')')
            ),
            multispace0
        )(i) {
            Ok((next_i, args)) => {
                i = next_i;
                expr = Expression::Invocation(Box::new(InvocationExpression {
                    callee: Box::new(expr),
                    arguments: args,
                }));
                continue;
            },
            Err(_) => {}
        }
        
        // Try to parse a member access
        match tuple((
            multispace0,
            bchar('.'),
            multispace0,
            map(nom_to_bs(parse_identifier), |id| id)
        ))(i) {
            Ok((next_i, (_, _, _, name))) => {
                i = next_i;
                expr = Expression::MemberAccess(Box::new(MemberAccessExpression {
                    object: Box::new(expr),
                    member: name,
                }));
                continue;
            },
            Err(_) => {}
        }
        
        // No more postfix operators, break the loop
        break;
    }
    
    Ok((i, expr))
}

fn parse_primary_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "primary expression",
        alt((
            map(nom_to_bs(parse_boolean), Expression::Literal),
            map(nom_to_bs(parse_char_literal), Expression::Literal),
            map(nom_to_bs(parse_integer), Expression::Literal), 
            map(nom_to_bs(parse_string), Expression::Literal),
            
            map(nom_to_bs(parse_identifier), |id| Expression::Variable(id)),
            
            // 'this' keyword - creating a variable with name "this"
            map(keyword("this"), |_| Expression::Variable(Identifier { name: "this".to_string() })),
            
            map(
                delimited(
                    bws(bchar('(')), 
                    parse_expression, 
                    bws(bchar(')'))
                ),
                // ExpressionSyntax within parentheses is just the expression itself
                |expr| expr
            ),

            // TODO: Add 'new' expressions (object creation, array creation)
            // TODO: Add member access, invocation, index access (postfix operators)

        ))
    )(input)
}

// Parse additive expressions (+ and -)
fn parse_additive_expression(input: &str) -> BResult<&str, Expression> {
    let (input, initial) = parse_multiplicative_expression(input)?;
    
    fold_many0(
        tuple((
            multispace0,
            alt((
                map(bchar('+'), |_| BinaryOperator::Add),
                map(bchar('-'), |_| BinaryOperator::Subtract),
            )),
            multispace0,
            parse_multiplicative_expression
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| {
            Expression::Binary {
                left: Box::new(acc),
                op,
                right: Box::new(right),
            }
        }
    )(input)
}

// Parse multiplicative expressions (* and /)
fn parse_multiplicative_expression(input: &str) -> BResult<&str, Expression> {
    let (input, initial) = parse_postfix_expression(input)?;
    
    fold_many0(
        tuple((
            multispace0,
            alt((
                map(bchar('*'), |_| BinaryOperator::Multiply),
                map(bchar('/'), |_| BinaryOperator::Divide),
            )),
            multispace0,
            parse_postfix_expression
        )),
        move || initial.clone(),
        |acc, (_, op, _, right)| {
            Expression::Binary {
                left: Box::new(acc),
                op,
                right: Box::new(right),
            }
        }
    )(input)
}

// tests moved to tests/parser/expressions/expression_tests.rs
// #[cfg(test)]
// mod tests {
