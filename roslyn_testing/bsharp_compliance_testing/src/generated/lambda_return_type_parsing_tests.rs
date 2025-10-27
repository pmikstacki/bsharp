// Auto-generated from Roslyn: LambdaReturnTypeParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
/// Roslyn: LambdaReturnTypeParsingTests.NullablePointer (case 1)
#[test]
fn nullable_pointer() {
    let src = r#"int?* () => default"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { int?* () => default; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_return_type_parsing_tests",
                "LambdaReturnTypeParsingTests",
                "NullablePointer",
                1,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaReturnTypeParsingTests.ArrayPointer (case 2)
#[test]
fn array_pointer() {
    let src = r#"int[]* () => default"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { int[]* () => default; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_return_type_parsing_tests",
                "LambdaReturnTypeParsingTests",
                "ArrayPointer",
                2,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}
