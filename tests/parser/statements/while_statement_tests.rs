// Integration tests for while_statement_parser.rs
// Content moved from src/parser/statements/while_statement_parser.rs

use bsharp::parser::expressions::statements::while_statement_parser::parse_while_statement;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;
use bsharp::syntax::nodes::statements::statement::Statement;
use bsharp::syntax::test_helpers::parse_all;

#[test]
fn test_parse_while_statement() {
    let input = "while (true) { DoSomething(); }";
    let result = parse_all(parse_while_statement, input);
    assert!(result.is_ok());
    match result.unwrap().1 {
        Statement::While(boxed_while_stmt) => {
            let while_stmt = &*boxed_while_stmt;
            assert_eq!(
                *while_stmt.condition,
                Expression::Literal(Literal::Boolean(true))
            );
            // Check block structure if needed
            assert!(matches!(*while_stmt.body, Statement::Block(_)));
            if let Statement::Block(ref _b) = *while_stmt.body {
                // Check block contents if needed, e.g. _b.len()
            }
        }
        _ => panic!("Expected While statement"),
    }
}
