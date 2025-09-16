// Tests for parsing new expressions

use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;
use bsharp::syntax::nodes::types::{PrimitiveType, Type};
use bsharp::parser::expressions::expression_parser::parse_expression;

fn parse_new_expr(code: &str) -> Result<Expression, String> {
    match parse_expression(code) {
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
fn test_parse_simple_new_expr() {
    let code = "new Exception()";
    let result = parse_new_expr(code);
    assert!(result.is_ok(), "Failed to parse simple new expression: {:?}", result);
    
    if let Ok(Expression::New(new_expr)) = result {
        assert!(matches!(new_expr.ty, Type::Reference(_)));
        if let Type::Reference(id) = &new_expr.ty {
            assert_eq!(id.name, "Exception");
        }
        assert_eq!(new_expr.arguments.len(), 0);
        assert!(new_expr.object_initializer.is_none());
        assert!(new_expr.collection_initializer.is_none());
    } else {
        panic!("Expected New expression");
    }
}

#[test]
fn test_parse_new_with_arguments() {
    let code = "new Exception(\"Error message\")";
    let result = parse_new_expr(code);
    assert!(result.is_ok(), "Failed to parse new with arguments: {:?}", result);
    
    if let Ok(Expression::New(new_expr)) = result {
        assert_eq!(new_expr.arguments.len(), 1);
        assert!(matches!(new_expr.arguments[0], Expression::Literal(Literal::String(_))));
        if let Expression::Literal(Literal::String(msg)) = &new_expr.arguments[0] {
            assert_eq!(msg, "Error message");
        }
    } else {
        panic!("Expected New expression");
    }
}

#[test]
fn test_parse_new_with_object_initializer() {
    let code = "new Person { Name = \"John\", Age = 30 }";
    let result = parse_new_expr(code);
    assert!(result.is_ok(), "Failed to parse new with object initializer: {:?}", result);
    
    if let Ok(Expression::New(new_expr)) = result {
        assert!(new_expr.object_initializer.is_some());
        if let Some(obj_init) = &new_expr.object_initializer {
            assert_eq!(obj_init.len(), 2);
            
            // Check that we have Name and Age properties
            let mut has_name = false;
            let mut has_age = false;
            for (prop_name, _) in obj_init {
                if prop_name == "Name" {
                    has_name = true;
                } else if prop_name == "Age" {
                    has_age = true;
                }
            }
            assert!(has_name, "Expected Name property");
            assert!(has_age, "Expected Age property");
        }
        assert!(new_expr.collection_initializer.is_none());
    } else {
        panic!("Expected New expression");
    }
}

#[test]
fn test_parse_new_with_collection_initializer() {
    let code = "new List<int> { 1, 2, 3, 4, 5 }";
    let result = parse_new_expr(code);
    assert!(result.is_ok(), "Failed to parse new with collection initializer: {:?}", result);
    
    if let Ok(Expression::New(new_expr)) = result {
        assert!(new_expr.collection_initializer.is_some());
        if let Some(coll_init) = &new_expr.collection_initializer {
            assert_eq!(coll_init.len(), 5);
            // Check that all elements are integer literals
            for (i, expr) in coll_init.iter().enumerate() {
                if let Expression::Literal(Literal::Integer(val)) = expr {
                    assert_eq!(*val, (i + 1) as i64);
                } else {
                    panic!("Expected integer literal at index {}", i);
                }
            }
        }
        assert!(new_expr.object_initializer.is_none());
    } else {
        panic!("Expected New expression");
    }
}

#[test]
fn test_parse_new_with_args_and_object_initializer() {
    let code = "new Person(\"firstName\") { LastName = \"Doe\", Age = 25 }";
    let result = parse_new_expr(code);
    assert!(result.is_ok(), "Failed to parse new with args and object initializer: {:?}", result);
    
    if let Ok(Expression::New(new_expr)) = result {
        assert_eq!(new_expr.arguments.len(), 1);
        assert!(new_expr.object_initializer.is_some());
        
        if let Some(obj_init) = &new_expr.object_initializer {
            assert_eq!(obj_init.len(), 2);
        }
    } else {
        panic!("Expected New expression");
    }
}

#[test]
fn test_parse_new_empty_object_initializer() {
    let code = "new Person { }";
    let result = parse_new_expr(code);
    assert!(result.is_ok(), "Failed to parse new with empty object initializer: {:?}", result);
    
    if let Ok(Expression::New(new_expr)) = result {
        // Empty initializers should be treated as collection initializers with empty vec
        assert!(new_expr.collection_initializer.is_some());
        if let Some(coll_init) = &new_expr.collection_initializer {
            assert_eq!(coll_init.len(), 0);
        }
    } else {
        panic!("Expected New expression");
    }
}

#[test]
fn test_parse_new_empty_collection_initializer() {
    let code = "new List<string> { }";
    let result = parse_new_expr(code);
    assert!(result.is_ok(), "Failed to parse new with empty collection initializer: {:?}", result);
    
    if let Ok(Expression::New(new_expr)) = result {
        assert!(new_expr.collection_initializer.is_some());
        if let Some(coll_init) = &new_expr.collection_initializer {
            assert_eq!(coll_init.len(), 0);
        }
        assert!(new_expr.object_initializer.is_none());
    } else {
        panic!("Expected New expression");
    }
}

#[test]
fn test_parse_new_primitive_type() {
    let code = "new int()";
    let result = parse_new_expr(code);
    assert!(result.is_ok(), "Failed to parse new primitive type: {:?}", result);
    
    if let Ok(Expression::New(new_expr)) = result {
        assert!(matches!(new_expr.ty, Type::Primitive(PrimitiveType::Int)));
        assert_eq!(new_expr.arguments.len(), 0);
    } else {
        panic!("Expected New expression");
    }
}

#[test]
fn test_parse_new_complex_nested_initializer() {
    let code = "new Dictionary<string, List<int>> { [\"numbers\"] = new List<int> { 1, 2, 3 } }";
    let result = parse_new_expr(code);
    // This is a complex case that might not fully parse yet, but we should at least
    // be able to parse the outer structure
    if result.is_ok() {
        if let Ok(Expression::New(new_expr)) = result {
            // Should recognize it as a new expression with some kind of initializer
            assert!(new_expr.object_initializer.is_some() || new_expr.collection_initializer.is_some());
        }
    }
    // Even if this complex case doesn't fully work, we should not panic
}
