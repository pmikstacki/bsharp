// Tests for parsing empty statements

use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parsers::statements::empty_statement_parser::parse_empty_statement;

#[test]
fn test_parse_empty_statement_simple() {
    let input = ";";
    let result = parse_empty_statement(input);
    assert!(result.is_ok());
    let (rest, stmt) = result.unwrap();
    assert_eq!(rest, "");
    assert_eq!(stmt, Statement::Empty);
}

#[test]
fn test_parse_empty_statement_with_whitespace() {
    let input = "   ;   ";
    let result = parse_empty_statement(input);
    assert!(result.is_ok());
    let (rest, stmt) = result.unwrap();
    assert_eq!(rest, "");
    assert_eq!(stmt, Statement::Empty);
}

#[test]
fn test_parse_empty_statement_fails_no_semicolon() {
    let input = "abc";
    assert!(parse_empty_statement(input).is_err());
} 