// Integration tests for block_statement_parser.rs
// Content moved from src/parser/statements/block_statement_parser.rs

use bsharp::parser::expressions::statements::block_statement_parser::parse_block_statement;
use bsharp::syntax::nodes::declarations::LocalVariableDeclaration;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;
use bsharp::syntax::nodes::statements::statement::Statement;
use bsharp::syntax::nodes::types::{PrimitiveType, Type};
use bsharp::syntax::test_helpers::parse_all;
use nom::Finish;
use nom::combinator::all_consuming;

#[test]
fn test_parse_block_statement() {
    let input_empty = "{}";
    let result_empty = parse_all(parse_block_statement, input_empty);
    assert!(
        result_empty.is_ok(),
        "Parsing empty block failed: {:?}",
        result_empty.err()
    );
    match result_empty.unwrap().1 {
        Statement::Block(stmts) => assert!(stmts.is_empty(), "Block was not empty"),
        res => panic!("Expected empty Block statement, got {:?}", res),
    }

    let input_simple = "{ int x = 1; return x; }";
    let result_simple = parse_all(parse_block_statement, input_simple);
    assert!(
        result_simple.is_ok(),
        "Parsing simple block failed: {:?}",
        result_simple.err()
    );
    match result_simple.unwrap().1 {
        Statement::Block(stmts) => {
            println!("Found {} statements in block", stmts.len());
            for (i, stmt) in stmts.iter().enumerate() {
                println!("Statement {}: {:?}", i, stmt);
            }
            assert_eq!(stmts.len(), 2, "Simple block did not contain 2 statements");
            assert!(
                matches!(stmts[0], Statement::Declaration(_)),
                "First statement was not a Declaration"
            );
            assert!(
                matches!(stmts[1], Statement::Return(_)),
                "Second statement was not a Return"
            );
        }
        res => panic!("Expected Block statement, got {:?}", res),
    }

    let input_nested = "{ { x = 5; } } "; // Note: trailing space
    let result_nested = parse_all(parse_block_statement, input_nested.trim_end()); // Trim trailing space for all_consuming
    assert!(
        result_nested.is_ok(),
        "Parsing nested block failed: {:?}",
        result_nested.err()
    );
    match result_nested.unwrap().1 {
        Statement::Block(stmts) => {
            assert_eq!(stmts.len(), 1, "Outer block did not contain 1 statement");
            match &stmts[0] {
                Statement::Block(inner_stmts) => {
                    assert_eq!(
                        inner_stmts.len(),
                        1,
                        "Inner block did not contain 1 statement"
                    );
                    // Assuming "x = 5;" parses as an ExpressionStatement
                    assert!(
                        matches!(inner_stmts[0], Statement::Expression(_)),
                        "Innermost statement was not an Expression"
                    );
                }
                _ => panic!("Inner statement was not a Block as expected"),
            }
        }
        res => panic!("Expected Block statement for nested, got {:?}", res),
    }
}

#[test]
fn test_parse_block_statement_empty() {
    let input = "{}";
    let result = all_consuming(parse_block_statement)(input).finish();
    assert!(
        result.is_ok(),
        "Parsing empty block failed: {:?}",
        result.err()
    );
    match result.unwrap().1 {
        Statement::Block(statements) => {
            assert!(statements.is_empty(), "Block was not empty");
        }
        _ => panic!("Expected Statement::Block for empty block"),
    }
}

#[test]
fn test_parse_block_statement_with_statements() {
    let input = "{ int a = 1; string b = \"test\"; }";
    let result = all_consuming(parse_block_statement)(input).finish();
    assert!(
        result.is_ok(),
        "Parsing block with statements failed: {:?}",
        result.err()
    );

    match result.unwrap().1 {
        Statement::Block(statements) => {
            assert_eq!(statements.len(), 2, "Block did not contain 2 statements");

            // Validate first statement: int a = 1;
            match &statements[0] {
                Statement::Declaration(LocalVariableDeclaration {
                    declarators,
                    declaration_type: ty,
                    ..
                }) => {
                    assert_eq!(
                        declarators.len(),
                        1,
                        "Expected one declarator for 'int a = 1;'"
                    );
                    let declarator = &declarators[0];
                    assert_eq!(declarator.name.name, "a");
                    assert!(matches!(
                        declarator.initializer,
                        Some(Expression::Literal(Literal::Integer(1)))
                    ));
                    assert!(matches!(ty, Type::Primitive(PrimitiveType::Int)));
                }
                _ => panic!("Expected first statement to be a LocalVariableDeclaration"),
            }

            // Validate second statement: string b = "test";
            match &statements[1] {
                Statement::Declaration(LocalVariableDeclaration {
                    declarators,
                    declaration_type: ty,
                    ..
                }) => {
                    assert_eq!(
                        declarators.len(),
                        1,
                        "Expected one declarator for 'string b = \"test\";'"
                    );
                    let declarator = &declarators[0];
                    assert_eq!(declarator.name.name, "b");
                    assert!(
                        matches!(&declarator.initializer, Some(Expression::Literal(Literal::String(s))) if s == "test")
                    );
                    assert!(matches!(ty, Type::Primitive(PrimitiveType::String)));
                }
                _ => panic!("Expected second statement to be a LocalVariableDeclaration"),
            }
        }
        _ => panic!("Expected Statement::Block for block with statements"),
    }
}
