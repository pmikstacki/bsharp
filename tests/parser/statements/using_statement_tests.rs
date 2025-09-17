// Tests for parsing using statements

use bsharp::parser::expressions::statements::using_statement_parser::parse_using_statement;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::statements::statement::Statement;

#[test]
fn test_simple_using_statement() {
    let input = "using (stream) { Console.WriteLine(\"Hello\"); }";
    let result = parse_using_statement(input);

    assert!(result.is_ok());
    let (remaining, statement) = result.unwrap();
    assert_eq!(remaining, "");

    match statement {
        Statement::Using(using_stmt) => {
            // Check that the resource is a variable expression
            match &using_stmt.resource {
                Expression::Variable(ident) => {
                    assert_eq!(ident.name, "stream");
                }
                _ => panic!("Expected variable expression for resource"),
            }

            // Check that the body is a block statement
            match &*using_stmt.body {
                Statement::Block(_) => {
                    // Expected
                }
                _ => panic!("Expected block statement for body"),
            }
        }
        _ => panic!("Expected using statement"),
    }
}

#[test]
fn test_using_statement_with_new_expression() {
    // Note: This test assumes we can parse new expressions
    let input = "using (new FileStream()) { }";
    let result = parse_using_statement(input);

    assert!(result.is_ok());
    let (remaining, statement) = result.unwrap();
    assert_eq!(remaining, "");

    match statement {
        Statement::Using(using_stmt) => {
            // Check that we have a new expression
            match &using_stmt.resource {
                Expression::New(_) => {
                    // Expected
                }
                _ => panic!("Expected new expression for resource"),
            }
        }
        _ => panic!("Expected using statement"),
    }
}

#[test]
fn test_using_statement_with_block_body() {
    let input = r#"using (resource) 
    { 
        Console.WriteLine("Inside using block"); 
    }"#;
    let result = parse_using_statement(input);

    assert!(result.is_ok());
    let (remaining, _) = result.unwrap();
    assert_eq!(remaining, "");
}

#[test]
fn test_using_statement_with_single_statement_body() {
    let input = "using (resource) Console.WriteLine(\"test\");";
    let result = parse_using_statement(input);

    assert!(result.is_ok());
    let (remaining, statement) = result.unwrap();
    assert_eq!(remaining, "");

    match statement {
        Statement::Using(using_stmt) => {
            // Body should be an expression statement
            match &*using_stmt.body {
                Statement::Expression(_) => {
                    // Expected
                }
                _ => panic!("Expected expression statement for body"),
            }
        }
        _ => panic!("Expected using statement"),
    }
}

#[test]
fn test_using_statement_fails_without_parentheses() {
    let input = "using resource { }";
    let result = parse_using_statement(input);

    assert!(result.is_err());
}

#[test]
fn test_using_statement_fails_without_body() {
    let input = "using (resource)";
    let result = parse_using_statement(input);

    assert!(result.is_err());
}
