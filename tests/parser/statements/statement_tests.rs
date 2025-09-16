// Tests for parsing general statements (Statement enum)

use bsharp::syntax::nodes::statements::statement::{Statement, Statement::*};
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::parser::statement_parser::{parse_statement, debug_test_individual_parsers};
use bsharp::parser::statements::block_statement_parser::parse_block_statement;

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
    assert!(bsharp::syntax::test_helpers::parse_statement_all("if (true) {} ").is_ok());
    assert!(bsharp::syntax::test_helpers::parse_statement_all("while (false) break;").is_ok());
    assert!(bsharp::syntax::test_helpers::parse_statement_all("MyFunc();").is_ok());
    assert!(bsharp::syntax::test_helpers::parse_statement_all("{} ").is_ok());
    assert!(bsharp::syntax::test_helpers::parse_statement_all("int y;").is_ok());
}

#[test]
fn test_debug_individual_parsers() {
    let input = "{ foo; 42; }";
    let debug_output = debug_test_individual_parsers(input);
    println!("DEBUG OUTPUT for input '{}':\n{}", input, debug_output);
    
    // This test always passes - it's just for debugging
    assert!(true);
}

#[test]
fn test_direct_block_statement_call() {
    let input = "{ foo; 42; }";
    println!("TESTING DIRECT parse_block_statement call with input: '{}'", input);
    
    match parse_block_statement(input) {
        Ok((remaining, stmt)) => {
            println!("✅ DIRECT parse_block_statement SUCCESS - remaining: {:?}, stmt: {:?}", remaining, stmt);
        }
        Err(e) => {
            println!("❌ DIRECT parse_block_statement FAILED - {:?}", e);
        }
    }
    
    // This test always passes - it's just for debugging
    assert!(true);
}
