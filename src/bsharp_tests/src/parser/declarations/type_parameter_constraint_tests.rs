// Tests for parsing type parameter constraints

use parser::expressions::declarations::type_parameter_parser::parse_type_parameter_where_clause;
use syntax::nodes::declarations::TypeParameterConstraint;

#[test]
fn test_parse_where_class() {
    let code = "where T : class";
    let (rest, clause) = parse_type_parameter_where_clause(code).expect("should parse where clause");
    assert!(rest.is_empty());
    assert_eq!(clause.type_param.name, "T");
    assert_eq!(clause.constraints.len(), 1);
    assert!(matches!(clause.constraints[0], TypeParameterConstraint::ReferenceType));
}

#[test]
fn test_parse_where_multiple_constraints() {
    let code = "where T : struct, new()";
    let (_, clause) = parse_type_parameter_where_clause(code).expect("should parse multiple constraints");
    assert_eq!(clause.type_param.name, "T");
    assert_eq!(clause.constraints.len(), 2);
}
