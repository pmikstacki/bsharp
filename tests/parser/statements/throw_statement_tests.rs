// Integration tests for throw_statement_parser.rs
// Content moved from src/parsers/statements/throw_statement_parser.rs

use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parser::test_helpers::parse_all;
use bsharp::parsers::statements::throw_statement_parser::parse_throw_statement;

#[test]
fn test_parse_throw_statement() {
    let input_no_expr = "throw;";
    let result_no_expr = parse_all(parse_throw_statement, input_no_expr);
    assert!(result_no_expr.is_ok());
    assert_eq!(result_no_expr.unwrap().1, Statement::Throw(None));

    let input_with_expr = "throw new Exception(\"Error\");";
    let result_with_expr = parse_all(parse_throw_statement, input_with_expr);
    assert!(result_with_expr.is_ok());
    match result_with_expr.unwrap().1 {
        Statement::Throw(Some(expr)) => {
            // Basic check, can add more details
            assert!(matches!(*expr, Expression::New(_)));
        }
        _ => panic!("Expected Throw statement with expression"),
    }
}
