// Integration tests for break_statement_parser.rs
// Content moved from src/parser/statements/break_statement_parser.rs

use parser::expressions::statements::break_statement_parser::parse_break_statement;
use parser::syntax::test_helpers::parse_all;
use syntax::statements::break_statement::BreakStatement;
use syntax::statements::statement::Statement;

#[test]
fn test_parse_break_statement() {
    let input = "break;";
    let result = parse_all(parse_break_statement, input.into());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().1, Statement::Break(BreakStatement {}));
}
