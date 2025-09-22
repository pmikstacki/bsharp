// Tests for parameter modifiers with full semantics

use bsharp::parser::expressions::declarations::parameter_parser::parse_parameter;
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::syntax::nodes::types::{Parameter, ParameterModifier, PrimitiveType, Type};

fn parse_parameter_test(code: &str) -> Result<Parameter, String> {
    match parse_parameter(code) {
        Ok((rest, param)) if rest.trim().is_empty() => Ok(param),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_ref_parameter() {
    let code = "ref int value";
    let expected = Parameter {
        attributes: vec![],
        modifier: Some(ParameterModifier::Ref),
        parameter_type: Type::Primitive(PrimitiveType::Int),
        name: Identifier::new("value"),
        default_value: None,
    };
    assert_eq!(parse_parameter_test(code), Ok(expected));
}

#[test]
fn test_parse_out_parameter() {
    let code = "out string result";
    let expected = Parameter {
        attributes: vec![],
        modifier: Some(ParameterModifier::Out),
        parameter_type: Type::Primitive(PrimitiveType::String),
        name: Identifier::new("result"),
        default_value: None,
    };
    assert_eq!(parse_parameter_test(code), Ok(expected));
}

#[test]
fn test_parse_in_parameter() {
    let code = "in BigStruct data";
    let expected = Parameter {
        attributes: vec![],
        modifier: Some(ParameterModifier::In),
        parameter_type: Type::Reference(Identifier::new("BigStruct")),
        name: Identifier::new("data"),
        default_value: None,
    };
    assert_eq!(parse_parameter_test(code), Ok(expected));
}

#[test]
fn test_parse_params_parameter() {
    let code = "params int[] values";
    let expected = Parameter {
        attributes: vec![],
        modifier: Some(ParameterModifier::Params),
        parameter_type: Type::Array {
            element_type: Box::new(Type::Primitive(PrimitiveType::Int)),
            rank: 1,
        },
        name: Identifier::new("values"),
        default_value: None,
    };
    assert_eq!(parse_parameter_test(code), Ok(expected));
}

#[test]
fn test_parse_regular_parameter() {
    let code = "double value";
    let expected = Parameter {
        attributes: vec![],
        modifier: None,
        parameter_type: Type::Primitive(PrimitiveType::Double),
        name: Identifier::new("value"),
        default_value: None,
    };
    assert_eq!(parse_parameter_test(code), Ok(expected));
}

#[test]
fn test_parameter_modifier_semantics() {
    // Test ref parameter semantics
    let ref_modifier = ParameterModifier::Ref;
    assert!(ref_modifier.is_by_reference());
    assert!(ref_modifier.requires_initialization());
    assert!(!ref_modifier.must_be_assigned());
    assert!(!ref_modifier.is_read_only());
    assert!(!ref_modifier.is_params());

    // Test out parameter semantics
    let out_modifier = ParameterModifier::Out;
    assert!(out_modifier.is_by_reference());
    assert!(!out_modifier.requires_initialization());
    assert!(out_modifier.must_be_assigned());
    assert!(!out_modifier.is_read_only());
    assert!(!out_modifier.is_params());

    // Test in parameter semantics
    let in_modifier = ParameterModifier::In;
    assert!(in_modifier.is_by_reference());
    assert!(in_modifier.requires_initialization());
    assert!(!in_modifier.must_be_assigned());
    assert!(in_modifier.is_read_only());
    assert!(!in_modifier.is_params());

    // Test params parameter semantics
    let params_modifier = ParameterModifier::Params;
    assert!(!params_modifier.is_by_reference());
    assert!(!params_modifier.requires_initialization());
    assert!(!params_modifier.must_be_assigned());
    assert!(!params_modifier.is_read_only());
    assert!(params_modifier.is_params());
}

#[test]
fn test_parameter_modifier_from_modifier() {
    use bsharp::syntax::nodes::declarations::Modifier;

    assert_eq!(
        ParameterModifier::from_modifier(&Modifier::Ref),
        Some(ParameterModifier::Ref)
    );
    assert_eq!(
        ParameterModifier::from_modifier(&Modifier::Out),
        Some(ParameterModifier::Out)
    );
    assert_eq!(
        ParameterModifier::from_modifier(&Modifier::In),
        Some(ParameterModifier::In)
    );
    assert_eq!(
        ParameterModifier::from_modifier(&Modifier::Params),
        Some(ParameterModifier::Params)
    );

    // Non-parameter modifiers should return None
    assert_eq!(ParameterModifier::from_modifier(&Modifier::Public), None);
    assert_eq!(ParameterModifier::from_modifier(&Modifier::Static), None);
}

#[test]
fn test_parse_parameter_with_complex_types() {
    // Test ref with generic type
    let code = "ref List<string> items";
    let result = parse_parameter_test(code);
    assert!(result.is_ok());
    let param = result.unwrap();
    assert_eq!(param.modifier, Some(ParameterModifier::Ref));
    assert_eq!(param.name.name, "items");

    // Test out with nullable type
    let code = "out int? result";
    let result = parse_parameter_test(code);
    assert!(result.is_ok());
    let param = result.unwrap();
    assert_eq!(param.modifier, Some(ParameterModifier::Out));
    assert_eq!(param.name.name, "result");
}

#[test]
fn test_parse_parameter_whitespace_variations() {
    let variations = [
        "ref int value",
        "ref  int  value",
        "  ref int value  ",
        "ref\tint\tvalue",
    ];

    for code in &variations {
        let result = parse_parameter_test(code);
        assert!(result.is_ok(), "Failed to parse: {}", code);
        let param = result.unwrap();
        assert_eq!(param.modifier, Some(ParameterModifier::Ref));
        assert_eq!(param.name.name, "value");
    }
}

#[test]
fn test_parameter_modifier_edge_cases() {
    // Test that we don't parse partial matches
    let invalid_cases = [
        "re int value",      // partial 'ref'
        "ou int value",      // partial 'out'
        "i int value",       // partial 'in'
        "param int value",   // partial 'params'
        "referee int value", // 'ref' as prefix
    ];

    for code in &invalid_cases {
        let result = parse_parameter_test(code);
        // These should either fail or parse without modifiers
        if let Ok(param) = result {
            assert_eq!(
                param.modifier, None,
                "Should not have modifier for: {}",
                code
            );
        }
    }
}
