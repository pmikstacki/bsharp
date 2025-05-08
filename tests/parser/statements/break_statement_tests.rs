// Integration tests for break_statement_parser.rs
// Content moved from src/parsers/statements/break_statement_parser.rs

use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parser::nodes::statements::break_statement::BreakStatement;
use bsharp::parser::test_helpers::parse_all;
use bsharp::parsers::statements::break_statement_parser::parse_break_statement;

#[test]
fn test_parse_break_statement() {
    let input = "break;";
    let result = parse_all(parse_break_statement, input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().1, Statement::Break(BreakStatement {}));
}
