// Tests for parsing type parameters
use bsharp::parser::nodes::types; 
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parsers::declarations::type_parameter_parser::parse_type_parameter_list;

fn parse_test(code: &str) -> Result<Vec<types::TypeParameter>, String> { 
    match parse_type_parameter_list(code) {
        Ok((rest, params)) if rest.trim().is_empty() => Ok(params),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_single_type_parameter() {
    let code = "<T>";
    let expected = vec![
        types::TypeParameter { 
            name: Identifier { name: "T".to_string() },
            variance: types::Variance::None, 
        }
    ];
    assert_eq!(parse_test(code), Ok(expected));
}

#[test]
fn test_parse_multiple_type_parameters() {
    let code = "<K, V>";
    let expected = vec![
        types::TypeParameter { 
            name: Identifier { name: "K".to_string() },
            variance: types::Variance::None, 
        },
        types::TypeParameter { 
            name: Identifier { name: "V".to_string() },
            variance: types::Variance::None, 
        },
    ];
    assert_eq!(parse_test(code), Ok(expected));
}

#[test]
fn test_parse_type_parameter_with_variance() {
    let code = "<in T, out U>";
    let expected = vec![
        types::TypeParameter { 
            name: Identifier { name: "T".to_string() },
            variance: types::Variance::In, 
        },
        types::TypeParameter { 
            name: Identifier { name: "U".to_string() },
            variance: types::Variance::Out, 
        },
    ];
    assert_eq!(parse_test(code), Ok(expected));
}

#[test]
fn test_parse_empty_type_parameter_list() {
    // This should technically fail parsing, as <> is not valid C#
    // The parser expects at least one identifier if <> are present.
    // However, let's ensure our list parser fails correctly.
    let code = "<>";
    assert!(parse_test(code).is_err());
}
