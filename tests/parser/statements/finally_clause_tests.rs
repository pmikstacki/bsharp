// Tests for parsing finally clauses

use bsharp::parser::nodes::statements::FinallyClause;

fn parse_finally_clause(code: &str) -> Result<FinallyClause, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_finally_clause() {
    let code = "finally { }";
    // let expected = ...;
    // assert_eq!(parse_finally_clause(code), Ok(expected));
    assert!(parse_finally_clause(code).is_err());
}
