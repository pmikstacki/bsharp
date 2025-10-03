// Integration tests for do_while_statement_parser.rs
// Content moved from src/parser/statements/do_while_statement_parser.rs

use parser::expressions::statements::do_while_statement_parser::parse_do_while_statement;
use parser::syntax::test_helpers::parse_all;
use syntax::nodes::expressions::expression::Expression;
use syntax::nodes::expressions::literal::Literal;
use syntax::nodes::statements::statement::Statement;

#[test]
fn test_parse_do_while_statement() {
    let input = "do { Print(\"Hello\"); } while (false);";
    let result = parse_all(parse_do_while_statement, input);
    assert!(result.is_ok());
    match result.unwrap().1 {
        Statement::DoWhile(do_while_stmt) => {
            assert_eq!(
                do_while_stmt.condition,
                Expression::Literal(Literal::Boolean(false))
            );
            // Check block structure if needed
            assert!(matches!(*do_while_stmt.body, Statement::Block(_)));
            if let Statement::Block(_b) = *do_while_stmt.body {
                // Check block contents if needed
            }
        }
        _ => panic!("Expected DoWhile statement"),
    }
}
