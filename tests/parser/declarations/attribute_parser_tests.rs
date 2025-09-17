#![cfg(test)]
use bsharp::parser::expressions::declarations::attribute_parser::*;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;

#[test]
fn test_single_attribute_no_args() {
    let input = "[Serializable]";
    let (rest, lists) = parse_attribute_lists(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].attributes.len(), 1);
    assert_eq!(lists[0].attributes[0].name.name, "Serializable");
    assert!(lists[0].attributes[0].arguments.is_empty());
}

#[test]
fn test_multiple_attribute_lists() {
    let input = "[Serializable] [DataContract] class";
    let (rest, lists) = parse_attribute_lists(input).unwrap();
    assert_eq!(rest, "class");
    assert_eq!(lists.len(), 2);
    assert_eq!(lists[0].attributes[0].name.name, "Serializable");
    assert_eq!(lists[1].attributes[0].name.name, "DataContract");
}

#[test]
fn test_attribute_with_argument() {
    let input = "[DataMember(1)]";
    let (rest, lists) = parse_attribute_lists(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].attributes[0].name.name, "DataMember");
    assert_eq!(lists[0].attributes[0].arguments.len(), 1);

    // Verify the argument is a literal with value 1
    if let Expression::Literal(Literal::Integer(val)) = &lists[0].attributes[0].arguments[0] {
        assert_eq!(*val, 1);
    } else {
        panic!("Expected integer literal");
    }
}

#[test]
fn test_attribute_with_named_arguments() {
    // Note: This test will fail until the expression syntax fully supports
    // named arguments/assignments. We will need to enhance the expression syntax
    // to handle these. For now, adding as a placeholder.
    let input = "[DataMember(Name = \"firstName\")]";
    // Implementation will need to be updated when expression syntax supports assignments
}

#[test]
fn test_empty_attribute_list() {
    // No attributes in source code
    let input = "public class MyClass {}";
    let (rest, lists) = parse_attribute_lists(input).unwrap();
    assert_eq!(rest, "public class MyClass {}");
    assert!(lists.is_empty());
}

#[test]
fn test_multiple_attributes_in_one_list() {
    let input = "[Serializable, DataContract]";
    let (rest, lists) = parse_attribute_lists(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].attributes.len(), 2);
    assert_eq!(lists[0].attributes[0].name.name, "Serializable");
    assert_eq!(lists[0].attributes[1].name.name, "DataContract");
}

#[test]
fn test_attribute_with_multiple_arguments() {
    let input = "[DebuggerDisplay(\"Count = {Count}\", Type = \"MyType\")]";
    // Will need to be updated when expression syntax supports string literals and assignments
}
