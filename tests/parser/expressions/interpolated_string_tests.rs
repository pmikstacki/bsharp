use bsharp::parser::nodes::expressions::literal::{Literal, InterpolatedStringLiteral, InterpolatedStringPart};
use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parsers::expressions::literal_parser::parse_literal;

fn parse_interpolated_string_test(code: &str) -> Result<Literal, String> {
    match parse_literal(code) {
        Ok((remaining, literal)) => {
            if remaining.trim().is_empty() {
                Ok(literal)
            } else {
                Err(format!("Unparsed input: {}", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_interpolated_string() {
    let code = r#"$"Hello {name}""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse simple interpolated string: {:?}", result);
    
    if let Ok(Literal::InterpolatedString(interpolated)) = result {
        assert!(!interpolated.is_verbatim);
        assert_eq!(interpolated.parts.len(), 2);
        
        // First part should be text "Hello "
        if let InterpolatedStringPart::Text(text) = &interpolated.parts[0] {
            assert_eq!(text, "Hello ");
        } else {
            panic!("Expected text part, got: {:?}", interpolated.parts[0]);
        }
        
        // Second part should be interpolation {name}
        if let InterpolatedStringPart::Interpolation { expression, .. } = &interpolated.parts[1] {
            if let Expression::Variable(var) = expression {
                assert_eq!(var.name, "name");
            } else {
                panic!("Expected variable expression, got: {:?}", expression);
            }
        } else {
            panic!("Expected interpolation part, got: {:?}", interpolated.parts[1]);
        }
    } else {
        panic!("Expected interpolated string literal");
    }
}

#[test]
fn test_parse_verbatim_interpolated_string() {
    let code = r#"$@"Path: {path}""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse verbatim interpolated string: {:?}", result);
    
    if let Ok(Literal::InterpolatedString(interpolated)) = result {
        assert!(interpolated.is_verbatim);
        assert_eq!(interpolated.parts.len(), 2);
    } else {
        panic!("Expected interpolated string literal");
    }
}

#[test]
fn test_parse_alternative_verbatim_interpolated_string() {
    let code = r#"@$"Path: {path}""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse alternative verbatim interpolated string: {:?}", result);
    
    if let Ok(Literal::InterpolatedString(interpolated)) = result {
        assert!(interpolated.is_verbatim);
        assert_eq!(interpolated.parts.len(), 2);
    } else {
        panic!("Expected interpolated string literal");
    }
}

#[test]
fn test_parse_interpolated_string_with_multiple_expressions() {
    let code = r#"$"Hello {firstName} {lastName}!""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse interpolated string with multiple expressions: {:?}", result);
    
    if let Ok(Literal::InterpolatedString(interpolated)) = result {
        assert_eq!(interpolated.parts.len(), 5); // "Hello ", {firstName}, " ", {lastName}, "!"
    } else {
        panic!("Expected interpolated string literal");
    }
}

#[test]
fn test_parse_interpolated_string_with_format_specifier() {
    let code = r#"$"Price: {price:F2}""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse interpolated string with format specifier: {:?}", result);
    
    if let Ok(Literal::InterpolatedString(interpolated)) = result {
        assert_eq!(interpolated.parts.len(), 2);
        
        if let InterpolatedStringPart::Interpolation { format_string, .. } = &interpolated.parts[1] {
            assert_eq!(format_string.as_ref().unwrap(), "F2");
        } else {
            panic!("Expected interpolation with format string");
        }
    } else {
        panic!("Expected interpolated string literal");
    }
}

#[test]
fn test_parse_interpolated_string_with_alignment() {
    let code = r#"$"Value: {value,10}""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse interpolated string with alignment: {:?}", result);
    
    if let Ok(Literal::InterpolatedString(interpolated)) = result {
        assert_eq!(interpolated.parts.len(), 2);
        
        if let InterpolatedStringPart::Interpolation { alignment, .. } = &interpolated.parts[1] {
            assert!(alignment.is_some());
            if let Some(Expression::Literal(Literal::Integer(10))) = alignment {
                // Expected
            } else {
                panic!("Expected integer literal for alignment");
            }
        } else {
            panic!("Expected interpolation with alignment");
        }
    } else {
        panic!("Expected interpolated string literal");
    }
}

#[test]
fn test_parse_empty_interpolated_string() {
    let code = r#"$"""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse empty interpolated string: {:?}", result);
    
    if let Ok(Literal::InterpolatedString(interpolated)) = result {
        assert_eq!(interpolated.parts.len(), 0);
    } else {
        panic!("Expected interpolated string literal");
    }
}

#[test]
fn test_parse_verbatim_string() {
    let code = r#"@"C:\Path\To\File""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse verbatim string: {:?}", result);
    
    if let Ok(Literal::VerbatimString(content)) = result {
        assert_eq!(content, r"C:\Path\To\File");
    } else {
        panic!("Expected verbatim string literal");
    }
}

#[test]
fn test_parse_raw_string() {
    let code = r#""""This is a raw string""""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse raw string: {:?}", result);
    
    if let Ok(Literal::RawString(content)) = result {
        assert_eq!(content, "This is a raw string");
    } else {
        panic!("Expected raw string literal");
    }
}

#[test]
fn test_parse_regular_string_still_works() {
    let code = r#""Hello, World!""#;
    let result = parse_interpolated_string_test(code);
    assert!(result.is_ok(), "Failed to parse regular string: {:?}", result);
    
    if let Ok(Literal::String(content)) = result {
        assert_eq!(content, "Hello, World!");
    } else {
        panic!("Expected regular string literal");
    }
} 