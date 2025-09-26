use bsharp::parser::expressions::declarations::attribute_parser::parse_attribute_lists;

#[test]
fn parses_generic_attribute_simple() {
    let (rest, lists) = parse_attribute_lists("[MyAttr<int>]").expect("parse");
    assert!(rest.trim().is_empty());
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].attributes.len(), 1);
    assert_eq!(lists[0].attributes[0].name.name, "MyAttr<int>");
}

#[test]
fn parses_generic_attribute_with_namespace_and_args() {
    let (rest, lists) = parse_attribute_lists("[Ns.Attr<List<int>>(1, 2)]").expect("parse");
    assert!(rest.trim().is_empty());
    assert_eq!(lists.len(), 1);
    let attr = &lists[0].attributes[0];
    assert_eq!(attr.name.name, "Ns.Attr<List<int>>");
    assert_eq!(attr.arguments.len(), 2);
}

#[test]
fn parses_nested_generic_attribute() {
    let src = "[A.B.C<Dictionary<string, List<int>>>]";
    let (rest, lists) = parse_attribute_lists(src).expect("parse");
    assert!(rest.trim().is_empty());
    assert_eq!(lists[0].attributes[0].name.name, "A.B.C<Dictionary<string, List<int>>>");
}
