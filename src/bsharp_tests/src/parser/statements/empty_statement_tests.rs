// Tests for parsing empty statements

use parser::expressions::statements::empty_statement_parser::parse_empty_statement;
use syntax::statements::statement::Statement;

#[test]
fn test_parse_empty_statement_simple() {
    let input = ";";
    let result = parse_empty_statement(input.into());
    assert!(result.is_ok());
    let (rest, stmt) = result.unwrap();
    assert_eq!(rest.fragment().to_string(), "");
    assert_eq!(stmt, Statement::Empty);
}

#[test]
fn test_parse_empty_statement_with_whitespace() {
    let input = "   ;   ";
    let result = parse_empty_statement(input.into());
    assert!(result.is_ok());
    let (rest, stmt) = result.unwrap();
    assert_eq!(rest.fragment().to_string(), "");
    assert_eq!(stmt, Statement::Empty);
}

#[test]
fn test_parse_empty_statement_fails_no_semicolon() {
    let input = "abc";
    assert!(parse_empty_statement(input.into()).is_err());
}
