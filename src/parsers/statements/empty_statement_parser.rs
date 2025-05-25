use crate::parser::errors::BResult;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::parser_helpers::{bchar, bs_context, bws};
use nom::combinator::map;

// Parse an empty statement: ;
pub fn parse_empty_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "empty statement",
        map(bws(bchar(';')), |_| Statement::Empty),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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
} 