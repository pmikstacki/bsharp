// Tests for parsing local function statements

use bsharp::parser::nodes::declarations::{Modifier, TypeParameterConstraintClause};
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::statements::local_function_statement::LocalFunctionStatement;
use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parser::nodes::types::{Parameter, PrimitiveType, Type, TypeParameter, Variance};
use bsharp::parsers::statements::local_function_statement_parser::parse_local_function_statement;

fn parse_local_function_stmt(code: &str) -> Result<Statement, String> {
    match parse_local_function_statement(code) {
        Ok((rest, stmt)) if rest.trim().is_empty() => Ok(stmt),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_local_function() {
    let code = "int Add(int a, int b) { return a + b; }";
    let result = parse_local_function_stmt(code);
    assert!(result.is_ok(), "Failed to parse simple local function: {:?}", result);
    
    if let Ok(Statement::LocalFunction(local_func)) = result {
        assert_eq!(local_func.modifiers.len(), 0);
        assert_eq!(local_func.return_type, Type::Primitive(PrimitiveType::Int));
        assert_eq!(local_func.name.name, "Add");
        assert!(local_func.type_parameters.is_none());
        assert_eq!(local_func.parameters.len(), 2);
        assert!(local_func.constraints.is_none());
        assert!(matches!(*local_func.body, Statement::Block(_)));
        
        // Check parameters
        assert_eq!(local_func.parameters[0].parameter_type, Type::Primitive(PrimitiveType::Int));
        assert_eq!(local_func.parameters[0].name.name, "a");
        assert_eq!(local_func.parameters[1].parameter_type, Type::Primitive(PrimitiveType::Int));
        assert_eq!(local_func.parameters[1].name.name, "b");
    } else {
        panic!("Expected LocalFunction statement");
    }
}

#[test]
fn test_parse_void_local_function() {
    let code = "void PrintMessage(string message) { Console.WriteLine(message); }";
    let result = parse_local_function_stmt(code);
    assert!(result.is_ok(), "Failed to parse void local function: {:?}", result);
    
    if let Ok(Statement::LocalFunction(local_func)) = result {
        assert_eq!(local_func.return_type, Type::Primitive(PrimitiveType::Void));
        assert_eq!(local_func.name.name, "PrintMessage");
        assert_eq!(local_func.parameters.len(), 1);
        assert_eq!(local_func.parameters[0].parameter_type, Type::Primitive(PrimitiveType::String));
        assert_eq!(local_func.parameters[0].name.name, "message");
    } else {
        panic!("Expected LocalFunction statement");
    }
}

#[test]
fn test_parse_local_function_with_modifiers() {
    let code = "static async Task<string> FetchDataAsync() { return await GetDataAsync(); }";
    let result = parse_local_function_stmt(code);
    assert!(result.is_ok(), "Failed to parse local function with modifiers: {:?}", result);
    
    if let Ok(Statement::LocalFunction(local_func)) = result {
        assert_eq!(local_func.modifiers.len(), 2);
        assert!(local_func.modifiers.contains(&Modifier::Static));
        assert!(local_func.modifiers.contains(&Modifier::Async));
        assert_eq!(local_func.name.name, "FetchDataAsync");
    } else {
        panic!("Expected LocalFunction statement");
    }
}

#[test]
fn test_parse_generic_local_function() {
    let code = "T Identity<T>(T value) { return value; }";
    let result = parse_local_function_stmt(code);
    assert!(result.is_ok(), "Failed to parse generic local function: {:?}", result);
    
    if let Ok(Statement::LocalFunction(local_func)) = result {
        assert_eq!(local_func.name.name, "Identity");
        assert!(local_func.type_parameters.is_some());
        
        let type_params = local_func.type_parameters.unwrap();
        assert_eq!(type_params.len(), 1);
        assert_eq!(type_params[0].name.name, "T");
        assert_eq!(type_params[0].variance, Variance::None);
        
        // Check that the return type and parameter type reference T
        assert!(matches!(local_func.return_type, Type::Reference(_)));
        assert_eq!(local_func.parameters.len(), 1);
        assert!(matches!(local_func.parameters[0].parameter_type, Type::Reference(_)));
    } else {
        panic!("Expected LocalFunction statement");
    }
}

#[test]
fn test_parse_local_function_with_constraints() {
    let code = "T Process<T>(T item) where T : class { return item; }";
    let result = parse_local_function_stmt(code);
    assert!(result.is_ok(), "Failed to parse local function with constraints: {:?}", result);
    
    if let Ok(Statement::LocalFunction(local_func)) = result {
        assert_eq!(local_func.name.name, "Process");
        assert!(local_func.type_parameters.is_some());
        assert!(local_func.constraints.is_some());
        
        let constraints = local_func.constraints.unwrap();
        assert_eq!(constraints.len(), 1);
        assert_eq!(constraints[0].type_param.name, "T");
    } else {
        panic!("Expected LocalFunction statement");
    }
}

#[test]
fn test_parse_local_function_no_parameters() {
    let code = "string GetDefaultValue() { return \"default\"; }";
    let result = parse_local_function_stmt(code);
    assert!(result.is_ok(), "Failed to parse local function with no parameters: {:?}", result);
    
    if let Ok(Statement::LocalFunction(local_func)) = result {
        assert_eq!(local_func.name.name, "GetDefaultValue");
        assert_eq!(local_func.parameters.len(), 0);
        assert_eq!(local_func.return_type, Type::Primitive(PrimitiveType::String));
    } else {
        panic!("Expected LocalFunction statement");
    }
}

#[test]
fn test_parse_local_function_expression_body() {
    let code = "int Square(int x) => x * x;";
    let result = parse_local_function_stmt(code);
    assert!(result.is_ok(), "Failed to parse local function with expression body: {:?}", result);
    
    if let Ok(Statement::LocalFunction(local_func)) = result {
        assert_eq!(local_func.name.name, "Square");
        assert_eq!(local_func.parameters.len(), 1);
        // Expression body gets parsed as None for now (simplified implementation)
        assert!(matches!(*local_func.body, Statement::Empty));
    } else {
        panic!("Expected LocalFunction statement");
    }
}

#[test]
fn test_parse_local_function_multiple_parameters() {
    let code = "double Calculate(double x, double y, string operation) { return x + y; }";
    let result = parse_local_function_stmt(code);
    assert!(result.is_ok(), "Failed to parse local function with multiple parameters: {:?}", result);
    
    if let Ok(Statement::LocalFunction(local_func)) = result {
        assert_eq!(local_func.name.name, "Calculate");
        assert_eq!(local_func.parameters.len(), 3);
        
        assert_eq!(local_func.parameters[0].parameter_type, Type::Primitive(PrimitiveType::Double));
        assert_eq!(local_func.parameters[0].name.name, "x");
        
        assert_eq!(local_func.parameters[1].parameter_type, Type::Primitive(PrimitiveType::Double));
        assert_eq!(local_func.parameters[1].name.name, "y");
        
        assert_eq!(local_func.parameters[2].parameter_type, Type::Primitive(PrimitiveType::String));
        assert_eq!(local_func.parameters[2].name.name, "operation");
    } else {
        panic!("Expected LocalFunction statement");
    }
}

#[test]
fn test_parse_local_function_complex_generic() {
    let code = "TResult Transform<TInput, TResult>(TInput input) where TInput : class where TResult : new() { return default(TResult); }";
    let result = parse_local_function_stmt(code);
    assert!(result.is_ok(), "Failed to parse complex generic local function: {:?}", result);
    
    if let Ok(Statement::LocalFunction(local_func)) = result {
        assert_eq!(local_func.name.name, "Transform");
        assert!(local_func.type_parameters.is_some());
        assert!(local_func.constraints.is_some());
        
        let type_params = local_func.type_parameters.unwrap();
        assert_eq!(type_params.len(), 2);
        assert_eq!(type_params[0].name.name, "TInput");
        assert_eq!(type_params[1].name.name, "TResult");
        
        let constraints = local_func.constraints.unwrap();
        assert_eq!(constraints.len(), 2);
    } else {
        panic!("Expected LocalFunction statement");
    }
}

#[test]
fn test_parse_local_function_whitespace_variations() {
    let variations = vec![
        "int Add(int a, int b) { return a + b; }",
        "int  Add  (  int  a  ,  int  b  )  {  return a + b;  }",
        "static void Helper() { }",
        "static  void  Helper  (  )  {  }",
    ];
    
    for code in variations {
        let result = parse_local_function_stmt(code);
        assert!(result.is_ok(), "Failed to parse local function with whitespace variation: '{}' -> {:?}", code, result);
    }
}

#[test]
fn test_parse_local_function_errors() {
    let invalid_cases = vec![
        "Add(int a, int b) { }", // Missing return type
        "int { return 0; }", // Missing name
        "int Add { return 0; }", // Missing parameter list
        "int Add() ", // Missing body
    ];
    
    for code in invalid_cases {
        let result = parse_local_function_stmt(code);
        assert!(result.is_err(), "Expected parsing to fail for: '{}'", code);
    }
} 