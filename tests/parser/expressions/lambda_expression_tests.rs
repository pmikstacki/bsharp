// Tests for parsing lambda expressions

use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::expressions::lambda_expression::{LambdaBody, LambdaParameterModifier};
use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parser::nodes::types::{PrimitiveType, Type};
use bsharp::parsers::expressions::lambda_expression_parser::*;
use bsharp::parsers::types::type_parser::parse_type_expression;

fn parse_lambda_expr(code: &str) -> Result<Expression, String> {
    match parse_lambda_expression(code) {
        Ok((remaining, expr)) => {
            if remaining.trim().is_empty() {
                Ok(expr)
            } else {
                Err(format!("Unparsed input: {}", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn parse_anon_method_expr(code: &str) -> Result<Expression, String> {
    match parse_anonymous_method_expression(code) {
        Ok((remaining, expr)) => {
            if remaining.trim().is_empty() {
                Ok(expr)
            } else {
                Err(format!("Unparsed input: {}", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_lambda_expr() {
    let code = "x => x * 2";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse simple lambda: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert_eq!(lambda.parameters.len(), 1);
        assert_eq!(lambda.parameters[0].name.name, "x");
        assert!(lambda.parameters[0].ty.is_none());
        assert!(lambda.parameters[0].modifier.is_none());
        assert!(!lambda.is_async);
        assert!(matches!(lambda.body, LambdaBody::ExpressionSyntax(_)));
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_with_parentheses() {
    let code = "(x) => x + 1";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with parentheses: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert_eq!(lambda.parameters.len(), 1);
        assert_eq!(lambda.parameters[0].name.name, "x");
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_multiple_parameters() {
    let code = "(x, y) => x + y";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with multiple parameters: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert_eq!(lambda.parameters.len(), 2);
        assert_eq!(lambda.parameters[0].name.name, "x");
        assert_eq!(lambda.parameters[1].name.name, "y");
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_with_types() {
    let code = "(int x, string y) => x.ToString() + y";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with typed parameters: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert_eq!(lambda.parameters.len(), 2);
        assert_eq!(lambda.parameters[0].name.name, "x");
        assert!(lambda.parameters[0].ty.is_some());
        
        println!("First parameter type: {:?}", lambda.parameters[0].ty);
        
        if let Some(Type::Primitive(PrimitiveType::Int)) = lambda.parameters[0].ty {
            // Expected
        } else {
            panic!("Expected int type for first parameter, got: {:?}", lambda.parameters[0].ty);
        }
        
        assert_eq!(lambda.parameters[1].name.name, "y");
        assert!(lambda.parameters[1].ty.is_some());
        
        println!("Second parameter type: {:?}", lambda.parameters[1].ty);
        
        if let Some(Type::Primitive(PrimitiveType::String)) = lambda.parameters[1].ty {
            // Expected
        } else {
            panic!("Expected string type for second parameter, got: {:?}", lambda.parameters[1].ty);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_async_lambda() {
    let code = "async x => await SomeMethodAsync(x)";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse async lambda: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert!(lambda.is_async);
        assert_eq!(lambda.parameters.len(), 1);
        assert_eq!(lambda.parameters[0].name.name, "x");
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_no_parameters() {
    let code = "() => 42";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse parameterless lambda: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert_eq!(lambda.parameters.len(), 0);
        assert!(!lambda.is_async);
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_with_ref_parameter() {
    let code = "(ref int x) => x++";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with ref parameter: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert_eq!(lambda.parameters.len(), 1);
        assert_eq!(lambda.parameters[0].name.name, "x");
        assert!(matches!(lambda.parameters[0].modifier, Some(LambdaParameterModifier::Ref)));
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_anonymous_method() {
    let code = "delegate(int x) { return x * 2; }";
    let result = parse_anon_method_expr(code);
    assert!(result.is_ok(), "Failed to parse anonymous method: {:?}", result);
    
    if let Ok(Expression::AnonymousMethod(anon_method)) = result {
        assert_eq!(anon_method.parameters.len(), 1);
        assert_eq!(anon_method.parameters[0].name.name, "x");
        assert!(!anon_method.is_async);
        assert!(matches!(anon_method.body, LambdaBody::Block(_)));
    } else {
        panic!("Expected AnonymousMethod expression");
    }
}

#[test]
fn test_parse_async_anonymous_method() {
    let code = "async delegate(int x) { return await ProcessAsync(x); }";
    let result = parse_anon_method_expr(code);
    assert!(result.is_ok(), "Failed to parse async anonymous method: {:?}", result);
    
    if let Ok(Expression::AnonymousMethod(anon_method)) = result {
        assert!(anon_method.is_async);
        assert_eq!(anon_method.parameters.len(), 1);
        assert_eq!(anon_method.parameters[0].name.name, "x");
    } else {
        panic!("Expected AnonymousMethod expression");
    }
}

#[test]
fn test_parse_anonymous_method_no_parameters() {
    let code = "delegate { return 42; }";
    let result = parse_anon_method_expr(code);
    assert!(result.is_ok(), "Failed to parse parameterless anonymous method: {:?}", result);
    
    if let Ok(Expression::AnonymousMethod(anon_method)) = result {
        assert_eq!(anon_method.parameters.len(), 0);
        assert!(!anon_method.is_async);
    } else {
        panic!("Expected AnonymousMethod expression");
    }
}

#[test]
fn debug_type_parsing() {
    let result = parse_type_expression("int");
    println!("Type parsing result for 'int': {:?}", result);
    
    let result2 = parse_type_expression("string");
    println!("Type parsing result for 'string': {:?}", result2);
}

#[test]
fn debug_lambda_parameter_parsing() {
    use bsharp::parsers::expressions::lambda_expression_parser::parse_lambda_or_anonymous_method;
    
    let result = parse_lambda_or_anonymous_method("(int x) => x");
    println!("Lambda parameter parsing result for '(int x) => x': {:?}", result);
}

#[test]
fn test_parse_lambda_with_block_body() {
    let code = "x => { return x * 2; }";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with block body: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert_eq!(lambda.parameters.len(), 1);
        assert_eq!(lambda.parameters[0].name.name, "x");
        assert!(!lambda.is_async);
        
        // Check that the body is a block with actual statements
        if let LambdaBody::Block(statements) = &lambda.body {
            assert_eq!(statements.len(), 1, "Expected 1 statement in lambda block body");
            
            // Check that it's a return statement
            if let Statement::Return(Some(return_expr)) = &statements[0] {
                // The return expression should be a binary expression (x * 2)
                assert!(matches!(**return_expr, Expression::Binary { .. }));
            } else {
                panic!("Expected return statement, got: {:?}", statements[0]);
            }
        } else {
            panic!("Expected Block body, got: {:?}", lambda.body);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_with_multiple_statements() {
    let code = "(x, y) => { int sum = x + y; return sum * 2; }";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with multiple statements: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert_eq!(lambda.parameters.len(), 2);
        
        if let LambdaBody::Block(statements) = &lambda.body {
            assert_eq!(statements.len(), 2, "Expected 2 statements in lambda block body");
            
            // First statement should be a variable declaration
            assert!(matches!(statements[0], Statement::Declaration(_)));
            
            // Second statement should be a return statement
            assert!(matches!(statements[1], Statement::Return(_)));
        } else {
            panic!("Expected Block body, got: {:?}", lambda.body);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_empty_block() {
    let code = "() => { }";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with empty block: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert_eq!(lambda.parameters.len(), 0);
        
        if let LambdaBody::Block(statements) = &lambda.body {
            assert_eq!(statements.len(), 0, "Expected empty block body");
        } else {
            panic!("Expected Block body, got: {:?}", lambda.body);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_async_lambda_with_block_body() {
    let code = "async x => { var result = await ProcessAsync(x); return result; }";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse async lambda with block body: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        assert!(lambda.is_async);
        assert_eq!(lambda.parameters.len(), 1);
        
        if let LambdaBody::Block(statements) = &lambda.body {
            assert_eq!(statements.len(), 2, "Expected 2 statements in async lambda block body");
        } else {
            panic!("Expected Block body, got: {:?}", lambda.body);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_anonymous_method_with_block_body() {
    let code = "delegate(int x) { Console.WriteLine(x); return x * 2; }";
    let result = parse_anon_method_expr(code);
    assert!(result.is_ok(), "Failed to parse anonymous method with block body: {:?}", result);
    
    if let Ok(Expression::AnonymousMethod(anon_method)) = result {
        assert_eq!(anon_method.parameters.len(), 1);
        assert_eq!(anon_method.parameters[0].name.name, "x");
        assert!(!anon_method.is_async);
        
        if let LambdaBody::Block(statements) = &anon_method.body {
            assert_eq!(statements.len(), 2, "Expected 2 statements in anonymous method block body");
            
            // First should be an expression statement
            assert!(matches!(statements[0], Statement::Expression(_)));
            
            // Second should be a return statement
            assert!(matches!(statements[1], Statement::Return(_)));
        } else {
            panic!("Expected Block body, got: {:?}", anon_method.body);
        }
    } else {
        panic!("Expected AnonymousMethod expression");
    }
}

#[test]
fn test_parse_lambda_with_complex_block() {
    let code = r#"x => {
        if (x > 0) {
            return x * 2;
        } else {
            return 0;
        }
    }"#;
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with complex block: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        if let LambdaBody::Block(statements) = &lambda.body {
            assert_eq!(statements.len(), 1, "Expected 1 statement (if statement) in lambda block body");
            
            // The statement should be an if statement
            assert!(matches!(statements[0], Statement::If(_)));
        } else {
            panic!("Expected Block body, got: {:?}", lambda.body);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_with_local_variables() {
    let code = "(a, b) => { var temp = a; a = b; b = temp; return a + b; }";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with local variables: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        if let LambdaBody::Block(statements) = &lambda.body {
            assert_eq!(statements.len(), 4, "Expected 4 statements in lambda block body");
            
            // Should have: var declaration, assignment, assignment, return
            assert!(matches!(statements[0], Statement::Declaration(_)));
            assert!(matches!(statements[1], Statement::Expression(_)));
            assert!(matches!(statements[2], Statement::Expression(_)));
            assert!(matches!(statements[3], Statement::Return(_)));
        } else {
            panic!("Expected Block body, got: {:?}", lambda.body);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_with_try_catch() {
    let code = "x => { try { return x; } catch { return 0; } }";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with try-catch: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        if let LambdaBody::Block(statements) = &lambda.body {
            assert_eq!(statements.len(), 1, "Expected 1 statement (try statement) in lambda block body");
            
            // The statement should be a try statement
            assert!(matches!(statements[0], Statement::Try(_)));
        } else {
            panic!("Expected Block body, got: {:?}", lambda.body);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_nested_lambda_expressions() {
    let code = "x => { return y => x + y; }";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse nested lambda expressions: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        if let LambdaBody::Block(statements) = &lambda.body {
            assert_eq!(statements.len(), 1, "Expected 1 statement in outer lambda");
            
            if let Statement::Return(Some(return_expr)) = &statements[0] {
                // The return expression should be another lambda
                assert!(matches!(**return_expr, Expression::Lambda(_)));
            } else {
                panic!("Expected return statement with lambda expression");
            }
        } else {
            panic!("Expected Block body, got: {:?}", lambda.body);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}

#[test]
fn test_parse_lambda_with_foreach_loop() {
    let code = "items => { foreach (var item in items) { Console.WriteLine(item); } }";
    let result = parse_lambda_expr(code);
    assert!(result.is_ok(), "Failed to parse lambda with foreach loop: {:?}", result);
    
    if let Ok(Expression::Lambda(lambda)) = result {
        if let LambdaBody::Block(statements) = &lambda.body {
            assert_eq!(statements.len(), 1, "Expected 1 statement (foreach) in lambda block body");
            
            // The statement should be a foreach statement
            assert!(matches!(statements[0], Statement::ForEach(_)));
        } else {
            panic!("Expected Block body, got: {:?}", lambda.body);
        }
    } else {
        panic!("Expected Lambda expression");
    }
}
