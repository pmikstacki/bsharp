// Tests for parsing type parameter constraints

use bsharp::syntax::nodes::declarations::TypeParameterConstraintClause;

fn parse_type_parameter_constraint(code: &str) -> Result<TypeParameterConstraintClause, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_type_parameter_constraint() {
    let code = "where T : class";
    // let expected = ...;
    // assert_eq!(parse_type_parameter_constraint(code), Ok(expected));
    assert!(parse_type_parameter_constraint(code).is_err());
}
