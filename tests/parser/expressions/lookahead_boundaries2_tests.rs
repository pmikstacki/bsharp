// Additional lookahead boundary tests to lock in disambiguation

use bsharp::parser::expressions::primary_expression_parser::parse_expression;

#[test]
fn dot_vs_range_and_float() {
    // Ensure member access '.' is not consumed as range '..' or float part wrongly
    let (_, _) = parse_expression("obj.Member").expect("parse ok");
    let (_, _) = parse_expression("1..2").expect("parse ok");
    let (_, _) = parse_expression("1.0").expect("parse ok");
}

#[test]
fn range_vs_member_boundary_examples() {
    // Range followed by member should not conflate tokens
    let (_, _) = parse_expression("(a..b).ToString()").expect("parse ok");
    // Range starting with dots is valid on its own
    let (_, _) = parse_expression("..b").expect("parse ok");
    // Member after paren primary
    let (_, _) = parse_expression("(x).Y").expect("parse ok");
}

#[test]
fn float_vs_range_boundary_examples() {
    // Floating-point literal
    let (_, _) = parse_expression("2.5").expect("parse ok");
    // Range from integer literal to name
    let (_, _) = parse_expression("2..end").expect("parse ok");
    // Ensure not interpreting 'a. b' as float when space present
    let (_, _) = parse_expression("a. b").expect("parse ok");
}

#[test]
fn null_conditional_vs_null_coalescing_vs_ternary() {
    let (_, _) = parse_expression("x?.Y").expect("parse ok");
    let (_, _) = parse_expression("x ?? y").expect("parse ok");
    let (_, _) = parse_expression("cond ? a : b").expect("parse ok");
}

#[test]
fn lambda_vs_generics_less_than() {
    let res = parse_expression("Func<int> f = x => x;");
    assert!(res.is_err(), "expected error for statement-like input, got: {:?}", res);
    // but a generic name followed by member access must parse as primary name
    let (_, _) = parse_expression("Result<User>.Ok(1)").expect("parse ok");
}
