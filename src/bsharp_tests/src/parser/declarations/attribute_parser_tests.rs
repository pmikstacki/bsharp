#![allow(unused_variables)]
#![cfg(test)]
use parser::expressions::declarations::attribute_parser::*;
use syntax::expressions::{Expression, Literal};

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
    let input = "[DataMember(Name = \"firstName\")]";
    let (rest, lists) =
        parse_attribute_lists(input).expect("should parse attribute with named arg");
    assert_eq!(rest, "");
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].attributes.len(), 1);
    let args = &lists[0].attributes[0].arguments;
    assert_eq!(args.len(), 1);
    match &args[0] {
        Expression::Assignment(assign) => {
            // target should be a variable 'Name'
            if let Expression::Variable(id) = &*assign.target {
                assert_eq!(id.name, "Name");
            } else {
                panic!("expected assignment target to be variable 'Name'");
            }
            // value should be string literal "firstName"
            if let Expression::Literal(Literal::String(s)) = &*assign.value {
                assert_eq!(s, "firstName");
            } else {
                panic!("expected assignment value to be string literal 'firstName'");
            }
        }
        other => panic!("expected assignment expression, got {:?}", other),
    }
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
    let (rest, lists) =
        parse_attribute_lists(input).expect("should parse attribute with mixed args");
    assert_eq!(rest, "");
    assert_eq!(lists.len(), 1);
    let args = &lists[0].attributes[0].arguments;
    assert_eq!(args.len(), 2);
}
