// Tests for parsing new expressions

use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::expressions::literal::Literal;
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::expressions::new_expression::NewExpression;

fn parse_new_expr(code: &str) -> Result<NewExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_new_expr() {
    let code = "new Foo()";
    // let expected = ...;
    // assert_eq!(parse_new_expr(code), Ok(expected));
    assert!(parse_new_expr(code).is_err());
}
