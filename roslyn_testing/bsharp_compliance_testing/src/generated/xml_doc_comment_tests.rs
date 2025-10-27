// Auto-generated from Roslyn: XmlDocCommentTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
/// Roslyn: XmlDocCommentTests.TestDocumentationComment (case 1)
#[test]
fn documentation_comment() {
    let src = r#"TypeName"#;
    let span = Span::new(src);
    let src2 = r#"class C { TypeName }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "xml_doc_comment_tests",
                "XmlDocCommentTests",
                "TestDocumentationComment",
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

/// Roslyn: XmlDocCommentTests.TestXmlSeeElementAndXmlSeeAlsoElement (case 2)
#[test]
fn xml_see_element_and_xml_see_also_element() {
    let src = r#"TypeName"#;
    let span = Span::new(src);
    let src2 = r#"class C { TypeName }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "xml_doc_comment_tests",
                "XmlDocCommentTests",
                "TestXmlSeeElementAndXmlSeeAlsoElement",
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

/// Roslyn: XmlDocCommentTests.TestXmlSeeElementAndXmlSeeAlsoElement (case 3)
#[test]
fn xml_see_element_and_xml_see_also_element_case_2() {
    let src = r#"TypeName2"#;
    let span = Span::new(src);
    let src2 = r#"class C { TypeName2 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "xml_doc_comment_tests",
                "XmlDocCommentTests",
                "TestXmlSeeElementAndXmlSeeAlsoElement",
                3,
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

/// Roslyn: XmlDocCommentTests.TestXmlRemarksElement (case 4)
#[test]
fn xml_remarks_element() {
    let src = r#"TypeName"#;
    let span = Span::new(src);
    let src2 = r#"class C { TypeName }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "xml_doc_comment_tests",
                "XmlDocCommentTests",
                "TestXmlRemarksElement",
                4,
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

/// Roslyn: XmlDocCommentTests.TestXmlExceptionElement (case 5)
#[test]
fn xml_exception_element() {
    let src = r#"InvalidOperationException"#;
    let span = Span::new(src);
    let src2 = r#"class C { InvalidOperationException }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "xml_doc_comment_tests",
                "XmlDocCommentTests",
                "TestXmlExceptionElement",
                5,
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

/// Roslyn: XmlDocCommentTests.TestXmlPermissionElement (case 6)
#[test]
fn xml_permission_element() {
    let src = r#"MyPermission"#;
    let span = Span::new(src);
    let src2 = r#"class C { MyPermission }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "xml_doc_comment_tests",
                "XmlDocCommentTests",
                "TestXmlPermissionElement",
                6,
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
