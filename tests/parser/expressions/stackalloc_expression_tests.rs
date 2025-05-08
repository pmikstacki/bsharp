// Tests for parsing stackalloc expressions

// use nom::error::{Error, ErrorKind};
// use bsharp::parser::nodes::types::{PrimitiveType, Type};
use bsharp::parser::nodes::expressions::StackAllocExpression;
// use bsharp::parsers::expressions::stackalloc_expression_parser::parse_stackalloc_expression;
// use bsharp::parsers::expressions::primary_expression_parser::parse_primary_expression;

fn parse_stackalloc_expr(code: &str) -> Result<StackAllocExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

/*
#[test]
fn test_parse_stackalloc_expr() {
    let code = "stackalloc int[10]";
    // let expected = ...;
    // assert_eq!(parse_stackalloc_expr(code), Ok(expected));
    assert!(parse_stackalloc_expr(code).is_err());
}
*/
