// Integration tests for continue_statement_parser.rs
// Content moved from src/parser/statements/continue_statement_parser.rs

use parser::expressions::statements::continue_statement_parser::parse_continue_statement;
use parser::syntax::test_helpers::parse_all;
use syntax::nodes::statements::continue_statement::ContinueStatement;
use syntax::nodes::statements::statement::Statement;

#[test]
fn test_parse_continue_statement() {
    let input = "continue;";
    let result = parse_all(parse_continue_statement, input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().1, Statement::Continue(ContinueStatement {}));
}
