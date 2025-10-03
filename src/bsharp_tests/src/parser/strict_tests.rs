use std::fs;

use parser::bsharp::parse_csharp_source_strict;
use nom::Finish;

#[test]
fn strict_parse_fails_on_missing_then_statement() {
    // tests/cs_test_cases/failure.cs contains: if (true) with no body
    let path = std::path::Path::new("tests/cs_test_cases/failure.cs");
    let src = fs::read_to_string(path).expect("read failure.cs");
    let res = parse_csharp_source_strict(&src).finish();
    assert!(res.is_err(), "strict parser should fail on malformed if without body");
}
