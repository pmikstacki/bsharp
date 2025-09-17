// Integration tests for continue_statement_parser.rs
// Content moved from src/parser/statements/continue_statement_parser.rs

use bsharp::parser::expressions::statements::continue_statement_parser::parse_continue_statement;
use bsharp::syntax::nodes::statements::continue_statement::ContinueStatement;
use bsharp::syntax::nodes::statements::statement::Statement;
use bsharp::syntax::test_helpers::parse_all;

#[test]
fn test_parse_continue_statement() {
    let input = "continue;";
    let result = parse_all(parse_continue_statement, input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().1, Statement::Continue(ContinueStatement {}));
}
