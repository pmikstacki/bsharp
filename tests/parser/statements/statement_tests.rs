// Tests for parsing general statements (Statement enum)

use bsharp::parser::nodes::statements::statement::{Statement, Statement::*};
use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::expressions::literal::Literal;
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parsers::statements::statement_parser::parse_statement;

fn parse_statement_test(code: &str) -> Result<Statement, String> {
    match parse_statement(code) {
        Ok((rest, stmt)) if rest.trim().is_empty() => Ok(stmt),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_expression_statement() {
    let code = "foo;";
    let expected = Expression(Expression::Variable(Identifier { name: "foo".to_string() }));
    assert_eq!(parse_statement_test(code), Ok(expected));

    let code2 = "42;";
    let expected2 = Expression(Expression::Literal(Literal::Integer(42)));
    assert_eq!(parse_statement_test(code2), Ok(expected2));
}

#[test]
fn test_parse_block_statement() {
    let code = "{ foo; 42; }";
    let expected = Block(vec![
        Expression(Expression::Variable(Identifier { name: "foo".to_string() })),
        Expression(Expression::Literal(Literal::Integer(42))),
    ]);
    assert_eq!(parse_statement_test(code), Ok(expected));
}

#[test]
fn test_parse_statement_dispatch() {
    assert!(bsharp::parser::test_helpers::parse_all_statement("if (true) {} ").is_ok());
    assert!(bsharp::parser::test_helpers::parse_all_statement("while (false) break;").is_ok());
    assert!(bsharp::parser::test_helpers::parse_all_statement("MyFunc();").is_ok());
    assert!(bsharp::parser::test_helpers::parse_all_statement("{} ").is_ok());
    assert!(bsharp::parser::test_helpers::parse_all_statement("int y;").is_ok());
}
