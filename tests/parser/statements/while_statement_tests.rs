// Integration tests for while_statement_parser.rs
// Content moved from src/parsers/statements/while_statement_parser.rs

use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::expressions::literal::Literal;
use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parser::test_helpers::parse_all;
use bsharp::parsers::statements::while_statement_parser::parse_while_statement;

#[test]
fn test_parse_while_statement() {
    let input = "while (true) { DoSomething(); }";
    let result = parse_all(parse_while_statement, input);
    assert!(result.is_ok());
    match result.unwrap().1 {
        Statement::While(boxed_while_stmt) => {
            let while_stmt = &*boxed_while_stmt;
            assert_eq!(*while_stmt.condition, Expression::Literal(Literal::Boolean(true)));
            // Check block structure if needed
            assert!(matches!(*while_stmt.body, Statement::Block(_)));
            if let Statement::Block(ref b) = *while_stmt.body {
                // Check block contents if needed, e.g. b.len()
            }
        }
        _ => panic!("Expected While statement"),
    }
}
