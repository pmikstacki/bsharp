// Integration tests for return_statement_parser.rs
// Content moved from src/parser/statements/return_statement_parser.rs

use parser::expressions::statements::return_statement_parser::parse_return_statement;
use parser::syntax::test_helpers::parse_all;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;
use syntax::statements::statement::Statement;

#[test]
fn test_parse_return_statement() {
    let input_no_expr = "return;";
    let result_no_expr = parse_all(parse_return_statement, input_no_expr);
    assert!(result_no_expr.is_ok());
    assert_eq!(result_no_expr.unwrap().1, Statement::Return(None));

    let input_with_expr = "return 42;";
    let result_with_expr = parse_all(parse_return_statement, input_with_expr);
    assert!(result_with_expr.is_ok());
    match result_with_expr.unwrap().1 {
        Statement::Return(Some(expr)) => {
            assert_eq!(*expr, Expression::Literal(Literal::Integer(42)));
        }
        _ => panic!("Expected Return statement with expression"),
    }
}
