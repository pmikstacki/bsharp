// Tests for parsing lambda expressions

use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::expressions::lambda_expression::{LambdaBody, LambdaParameterModifier};
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
