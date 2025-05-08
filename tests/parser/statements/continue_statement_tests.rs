// Integration tests for continue_statement_parser.rs
// Content moved from src/parsers/statements/continue_statement_parser.rs

use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parser::nodes::statements::continue_statement::ContinueStatement;
use bsharp::parser::test_helpers::parse_all;
use bsharp::parsers::statements::continue_statement_parser::parse_continue_statement;

#[test]
fn test_parse_continue_statement() {
    let input = "continue;";
    let result = parse_all(parse_continue_statement, input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().1, Statement::Continue(ContinueStatement {}));
}
