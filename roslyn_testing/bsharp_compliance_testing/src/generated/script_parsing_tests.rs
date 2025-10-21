// Auto-generated from Roslyn: ScriptParsingTests
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
/// Roslyn: ScriptParsingTests.MethodDeclarationAndMethodCall (case 1)
#[test]
fn method_declaration_and_method_call() {
    let src = r#"
void bar() { }
bar();
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "MethodDeclarationAndMethodCall", 1, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "MethodDeclarationAndMethodCall", 1, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "MethodDeclarationAndMethodCall", 1, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.FieldDeclarationError1 (case 2)
#[test]
fn field_declaration_error_1() {
    let src = r#"int x y;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "FieldDeclarationError1", 2, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "FieldDeclarationError1", 2, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "FieldDeclarationError1", 2, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.FieldDeclarationError2 (case 3)
#[test]
fn field_declaration_error_2() {
    let src = r#"int x y z;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "FieldDeclarationError2", 3, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "FieldDeclarationError2", 3, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "FieldDeclarationError2", 3, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewExpression (case 4)
#[test]
fn new_expression() {
    let src = r#"new[] { 1 };"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewExpression", 4, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewExpression", 4, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewExpression", 4, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewAnonymousTypeExpressionStatement (case 5)
#[test]
fn new_anonymous_type_expression_statement() {
    let src = r#"new { a = 1 };"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewAnonymousTypeExpressionStatement", 5, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewAnonymousTypeExpressionStatement", 5, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewAnonymousTypeExpressionStatement", 5, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewArrayExpressionStatement (case 6)
#[test]
fn new_array_expression_statement() {
    let src = r#"new T[5];"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewArrayExpressionStatement", 6, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewArrayExpressionStatement", 6, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewArrayExpressionStatement", 6, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewArrayExpressionWithInitializerAndPostFixExpressionStatement (case 7)
#[test]
fn new_array_expression_with_initializer_and_post_fix_expression_statement() {
    let src = r#"new int[] { }.Clone();"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewArrayExpressionWithInitializerAndPostFixExpressionStatement", 7, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewArrayExpressionWithInitializerAndPostFixExpressionStatement", 7, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewArrayExpressionWithInitializerAndPostFixExpressionStatement", 7, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_Method_WithBody (case 8)
#[test]
fn new_modifier_method_with_body() {
    let src = r#"new void Goo() { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_WithBody", 8, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_WithBody", 8, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_WithBody", 8, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_Method_ReturnsIdentifier (case 9)
#[test]
fn new_modifier_method_returns_identifier() {
    let src = r#"
new T Goo();
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsIdentifier", 9, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsIdentifier", 9, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsIdentifier", 9, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_Method_ReturnsArray (case 10)
#[test]
fn new_modifier_method_returns_array() {
    let src = r#"new int[] Goo();"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsArray", 10, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsArray", 10, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsArray", 10, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_Method_ReturnsPartialArray (case 11)
#[test]
fn new_modifier_method_returns_partial_array() {
    let src = r#"
new partial[] Goo();
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsPartialArray", 11, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsPartialArray", 11, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsPartialArray", 11, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_PartialMethod_ReturnsPredefined (case 12)
#[test]
fn new_modifier_partial_method_returns_predefined() {
    let src = r#"new partial "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialMethod_ReturnsPredefined", 12, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialMethod_ReturnsPredefined", 12, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialMethod_ReturnsPredefined", 12, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_Method_ReturnsPrimitive (case 13)
#[test]
fn new_modifier_method_returns_primitive() {
    let src = r#"new int Goo();"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsPrimitive", 13, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsPrimitive", 13, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_ReturnsPrimitive", 13, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_Indexer_ReturnsIdentifier (case 14)
#[test]
fn new_modifier_indexer_returns_identifier() {
    let src = r#"
new T this[int a] { get; }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Indexer_ReturnsIdentifier", 14, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Indexer_ReturnsIdentifier", 14, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Indexer_ReturnsIdentifier", 14, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_Indexer_ReturnsArray (case 15)
#[test]
fn new_modifier_indexer_returns_array() {
    let src = r#"
new T[] this[int a] { get; }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Indexer_ReturnsArray", 15, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Indexer_ReturnsArray", 15, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Indexer_ReturnsArray", 15, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_PartialIndexer (case 16)
#[test]
fn new_modifier_partial_indexer() {
    let src = r#"
new partial partial this[int i] { get; }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialIndexer", 16, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialIndexer", 16, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialIndexer", 16, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_WithOtherModifier (case 17)
#[test]
fn new_modifier_with_other_modifier() {
    let src = r#"new "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_WithOtherModifier", 17, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_WithOtherModifier", 17, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_WithOtherModifier", 17, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_Class (case 18)
#[test]
fn new_modifier_class() {
    let src = r#"
new class C { }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Class", 18, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Class", 18, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_Class", 18, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.NewModifier_PartialClass (case 19)
#[test]
fn new_modifier_partial_class() {
    let src = r#"
new partial class C { }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialClass", 19, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialClass", 19, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialClass", 19, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Using (case 20)
#[test]
fn using() {
    let src = r#"
using Goo;
using Goo.Bar;
using Goo = Bar;
using (var x = bar) { }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Using", 20, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Using", 20, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Using", 20, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Unsafe_Block (case 21)
#[test]
fn unsafe_block() {
    let src = r#"
unsafe { }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Block", 21, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Block", 21, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Block", 21, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Unsafe_Field (case 22)
#[test]
fn unsafe_field() {
    let src = r#"
unsafe int Goo;
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Field", 22, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Field", 22, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Field", 22, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Unsafe_Method (case 23)
#[test]
fn unsafe_method() {
    let src = r#"
unsafe void Goo() { }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Method", 23, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Method", 23, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Method", 23, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Unsafe_Property (case 24)
#[test]
fn unsafe_property() {
    let src = r#"
unsafe int Goo { get; }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Property", 24, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Property", 24, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Unsafe_Property", 24, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Fixed (case 25)
#[test]
fn fixed() {
    let src = r#"
fixed (int* a = b) { }
fixed int x[5];
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Fixed", 25, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Fixed", 25, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Fixed", 25, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Delegate1 (case 26)
#[test]
fn delegate_1() {
    let src = r#"
delegate { }();
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Delegate1", 26, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Delegate1", 26, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Delegate1", 26, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Delegate2 (case 27)
#[test]
fn delegate_2() {
    let src = r#"
delegate(){ }();
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Delegate2", 27, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Delegate2", 27, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Delegate2", 27, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Delegate3 (case 28)
#[test]
fn delegate_3() {
    let src = r#"
delegate void Goo();
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Delegate3", 28, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Delegate3", 28, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Delegate3", 28, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Indexer1 (case 29)
#[test]
fn indexer_1() {
    let src = r#"
bool this[int index]{} 
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer1", 29, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer1", 29, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer1", 29, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Indexer2 (case 30)
#[test]
fn indexer_2() {
    let src = r#"
public partial bool this[int index] {}
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer2", 30, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer2", 30, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer2", 30, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Indexer4 (case 31)
#[test]
fn indexer_4() {
    let src = r#"
new public bool this[int index] { get; }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer4", 31, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer4", 31, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer4", 31, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Indexer5 (case 32)
#[test]
fn indexer_5() {
    let src = r#"
new public bool this[int index] { get; }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer5", 32, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer5", 32, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Indexer5", 32, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.ExternAlias (case 33)
#[test]
fn extern_alias() {
    let src = r#"
extern alias Goo;
extern alias Goo();
extern alias Goo { get; }
extern alias Goo<T> { get; }
"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "ExternAlias", 33, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "ExternAlias", 33, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "ExternAlias", 33, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.PartialMethod (case 34)
#[test]
fn partial_method() {
    let src = r#"
partial void Goo();
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "PartialMethod", 34, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "PartialMethod", 34, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "PartialMethod", 34, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Attributes (case 35)
#[test]
fn attributes() {
    let src = r#"
[assembly: Goo]
[module: Bar]
[Goo]
void goo() { }
[Bar]
int x;
[Baz]
class C { }
[Baz]
struct C { }
[Baz]
enum C { }
[Baz]
delegate D();
"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Attributes", 35, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Attributes", 35, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Attributes", 35, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Fields (case 36)
#[test]
fn fields() {
    let src = r#"
int x;
volatile int x;
readonly int x;
static int x;
fixed int x[10];
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Fields", 36, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Fields", 36, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Fields", 36, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Multiplication_Interactive_Semicolon (case 37)
#[test]
fn multiplication_interactive_semicolon() {
    let src = r#"a * b;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Multiplication_Interactive_Semicolon", 37, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Multiplication_Interactive_Semicolon", 37, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Multiplication_Interactive_Semicolon", 37, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Multiplication_Interactive_NoSemicolon (case 38)
#[test]
fn multiplication_interactive_no_semicolon() {
    let src = r#"a * b"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Multiplication_Interactive_NoSemicolon", 38, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Multiplication_Interactive_NoSemicolon", 38, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Multiplication_Interactive_NoSemicolon", 38, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Multiplication_Complex (case 39)
#[test]
fn multiplication_complex() {
    let src = r#"a<t>.n * f(x)"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Multiplication_Complex", 39, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Multiplication_Complex", 39, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Multiplication_Complex", 39, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Semicolon1 (case 40)
#[test]
fn ternary_field_decl_semicolon_1() {
    let src = r#"T ? a;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon1", 40, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon1", 40, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon1", 40, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Semicolon2 (case 41)
#[test]
fn ternary_field_decl_semicolon_2() {
    let src = r#"T ? b, c = 1;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon2", 41, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon2", 41, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon2", 41, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Semicolon3 (case 42)
#[test]
fn ternary_field_decl_semicolon_3() {
    let src = r#"T ? b = d => { };"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon3", 42, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon3", 42, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon3", 42, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Semicolon4 (case 43)
#[test]
fn ternary_field_decl_semicolon_4() {
    let src = r#"T ? b = x ? y : z;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon4", 43, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon4", 43, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon4", 43, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Comma1 (case 44)
#[test]
fn ternary_field_decl_comma_1() {
    let src = r#"T ? a,"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Comma1", 44, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Comma1", 44, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Comma1", 44, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Comma2 (case 45)
#[test]
fn ternary_field_decl_comma_2() {
    let src = r#"T ? a = 1,"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Comma2", 45, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Comma2", 45, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Comma2", 45, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_PropertyDecl1 (case 46)
#[test]
fn ternary_property_decl_1() {
    let src = r#"T ? a {"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl1", 46, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl1", 46, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl1", 46, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_PropertyDecl2 (case 47)
#[test]
fn ternary_property_decl_2() {
    let src = r#"T ? a.b {"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl2", 47, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl2", 47, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl2", 47, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_PropertyDecl3 (case 48)
#[test]
fn ternary_property_decl_3() {
    let src = r#"T ? a<T>.b {"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl3", 48, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl3", 48, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl3", 48, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_PropertyDecl4 (case 49)
#[test]
fn ternary_property_decl_4() {
    let src = r#"T ? a<T?>.b<S>.c {"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl4", 49, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl4", 49, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_PropertyDecl4", 49, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl1 (case 50)
#[test]
fn ternary_method_decl_1() {
    let src = r#"T ? a() {"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl1", 50, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl1", 50, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl1", 50, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl1_Where (case 51)
#[test]
fn ternary_method_decl_1_where() {
    let src = r#"T ? a() where"#;
    let expected = Some(ExpectedDiagnostics { count: 4, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl1_Where", 51, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl1_Where", 51, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl1_Where", 51, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl2 (case 52)
#[test]
fn ternary_method_decl_2() {
    let src = r#"T ? a(T b"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl2", 52, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl2", 52, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl2", 52, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl3 (case 53)
#[test]
fn ternary_method_decl_3() {
    let src = r#"T ? a.b(T c"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl3", 53, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl3", 53, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl3", 53, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl4 (case 54)
#[test]
fn ternary_method_decl_4() {
    let src = r#"T ? a<A>.b<B>(C c"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl4", 54, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl4", 54, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl4", 54, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl5 (case 55)
#[test]
fn ternary_method_decl_5() {
    let src = r#"T ? a([Attr]C c"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl5", 55, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl5", 55, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl5", 55, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl6 (case 56)
#[test]
fn ternary_method_decl_6() {
    let src = r#"T ? a([Attr(a = b)]c"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl6", 56, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl6", 56, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl6", 56, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl7 (case 57)
#[test]
fn ternary_method_decl_7() {
    let src = r#"T ? a(out C c"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl7", 57, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl7", 57, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl7", 57, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl8 (case 58)
#[test]
fn ternary_method_decl_8() {
    let src = r#"T ? a(C[] a"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl8", 58, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl8", 58, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl8", 58, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl9 (case 59)
#[test]
fn ternary_method_decl_9() {
    let src = r#"T ? a(params"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl9", 59, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl9", 59, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl9", 59, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl10 (case 60)
#[test]
fn ternary_method_decl_10() {
    let src = r#"T ? a(out T ? b"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl10", 60, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl10", 60, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl10", 60, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl11 (case 61)
#[test]
fn ternary_method_decl_11() {
    let src = r#"T ? a(ref T ? b"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl11", 61, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl11", 61, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl11", 61, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl12 (case 62)
#[test]
fn ternary_method_decl_12() {
    let src = r#"T ? a(params T ? b"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl12", 62, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl12", 62, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl12", 62, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl13 (case 63)
#[test]
fn ternary_method_decl_13() {
    let src = r#"T ? a([Attr]T ? b"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl13", 63, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl13", 63, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl13", 63, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl14A (case 64)
#[test]
fn ternary_method_decl_14_a() {
    let src = r#"T ? a(T ? b,"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl14A", 64, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl14A", 64, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl14A", 64, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl14B (case 65)
#[test]
fn ternary_method_decl_14_b() {
    let src = r#"T ? a(T ? b)"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl14B", 65, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl14B", 65, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl14B", 65, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl15 (case 66)
#[test]
fn ternary_method_decl_15() {
    let src = r#"T ? a(T c)"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl15", 66, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl15", 66, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl15", 66, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl16 (case 67)
#[test]
fn ternary_method_decl_16() {
    let src = r#"T ? a(this c d"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl16", 67, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl16", 67, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl16", 67, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl17 (case 68)
#[test]
fn ternary_method_decl_17() {
    let src = r#"T ? a(ref out T a"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl17", 68, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl17", 68, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl17", 68, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl18 (case 69)
#[test]
fn ternary_method_decl_18() {
    let src = r#"T ? a(int a"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl18", 69, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl18", 69, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl18", 69, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl19 (case 70)
#[test]
fn ternary_method_decl_19() {
    let src = r#"T ? a(ref int a"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl19", 70, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl19", 70, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl19", 70, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl20 (case 71)
#[test]
fn ternary_method_decl_20() {
    let src = r#"T ? a(T a ="#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl20", 71, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl20", 71, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl20", 71, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl21 (case 72)
#[test]
fn ternary_method_decl_21() {
    let src = r#"T ? a(T[,] a"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl21", 72, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl21", 72, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl21", 72, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl22 (case 73)
#[test]
fn ternary_method_decl_22() {
    let src = r#"T ? a(T?[10] a)"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl22", 73, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl22", 73, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl22", 73, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl_GenericAmbiguity1 (case 74)
#[test]
fn ternary_method_decl_generic_ambiguity_1() {
    let src = r#"T ? m(a < b, c > d)"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl_GenericAmbiguity1", 74, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl_GenericAmbiguity1", 74, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl_GenericAmbiguity1", 74, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression1 (case 75)
#[test]
fn ternary_expression_1() {
    let src = r#"T ? 1"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression1", 75, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression1", 75, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression1", 75, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression2 (case 76)
#[test]
fn ternary_expression_2() {
    let src = r#"T ? a"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression2", 76, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression2", 76, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression2", 76, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression3 (case 77)
#[test]
fn ternary_expression_3() {
    let src = r#"T ? a."#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression3", 77, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression3", 77, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression3", 77, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression4 (case 78)
#[test]
fn ternary_expression_4() {
    let src = r#"T ? a["#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression4", 78, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression4", 78, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression4", 78, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression5 (case 79)
#[test]
fn ternary_expression_5() {
    let src = r#"T ? a<"#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression5", 79, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression5", 79, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression5", 79, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression6 (case 80)
#[test]
fn ternary_expression_6() {
    let src = r#"T ? a<b"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression6", 80, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression6", 80, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression6", 80, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression7 (case 81)
#[test]
fn ternary_expression_7() {
    let src = r#"T ? a<b>"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression7", 81, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression7", 81, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression7", 81, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression8 (case 82)
#[test]
fn ternary_expression_8() {
    let src = r#"T ? a<b,c>"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression8", 82, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression8", 82, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression8", 82, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression9 (case 83)
#[test]
fn ternary_expression_9() {
    let src = r#"T ? a<b>."#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression9", 83, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression9", 83, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression9", 83, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression10 (case 84)
#[test]
fn ternary_expression_10() {
    let src = r#"T ? a<b>.c"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression10", 84, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression10", 84, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression10", 84, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression11 (case 85)
#[test]
fn ternary_expression_11() {
    let src = r#"T ? a<b>.c("#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression11", 85, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression11", 85, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression11", 85, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression12 (case 86)
#[test]
fn ternary_expression_12() {
    let src = r#"T ? a("#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression12", 86, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression12", 86, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression12", 86, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression13 (case 87)
#[test]
fn ternary_expression_13() {
    let src = r#"T ? a.b("#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression13", 87, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression13", 87, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression13", 87, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression14 (case 88)
#[test]
fn ternary_expression_14() {
    let src = r#"T ? m(c"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression14", 88, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression14", 88, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression14", 88, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression15 (case 89)
#[test]
fn ternary_expression_15() {
    let src = r#"T ? m(c,"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression15", 89, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression15", 89, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression15", 89, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression16 (case 90)
#[test]
fn ternary_expression_16() {
    let src = r#"T ? m(c:"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression16", 90, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression16", 90, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression16", 90, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression17 (case 91)
#[test]
fn ternary_expression_17() {
    let src = r#"T ? m(c?"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression17", 91, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression17", 91, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression17", 91, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression18 (case 92)
#[test]
fn ternary_expression_18() {
    let src = r#"T ? m(c? a"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression18", 92, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression18", 92, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression18", 92, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression19 (case 93)
#[test]
fn ternary_expression_19() {
    let src = r#"T ? m(c? a ="#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression19", 93, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression19", 93, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression19", 93, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression20 (case 94)
#[test]
fn ternary_expression_20() {
    let src = r#"T ? m(c? a = b ?"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression20", 94, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression20", 94, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression20", 94, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression21 (case 95)
#[test]
fn ternary_expression_21() {
    let src = r#"T ? m()"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression21", 95, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression21", 95, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression21", 95, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression22 (case 96)
#[test]
fn ternary_expression_22() {
    let src = r#"T ? m(a)"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression22", 96, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression22", 96, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression22", 96, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression23 (case 97)
#[test]
fn ternary_expression_23() {
    let src = r#"T ? m();"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression23", 97, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression23", 97, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression23", 97, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression24 (case 98)
#[test]
fn ternary_expression_24() {
    let src = r#"T ? m(a);"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression24", 98, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression24", 98, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression24", 98, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression25 (case 99)
#[test]
fn ternary_expression_25() {
    let src = r#"T ? m(x: 1"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression25", 99, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression25", 99, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression25", 99, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression26 (case 100)
#[test]
fn ternary_expression_26() {
    let src = r#"T ? m(x: 1, y: a ? b : c)"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression26", 100, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression26", 100, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression26", 100, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression27 (case 101)
#[test]
fn ternary_expression_27() {
    let src = r#"T ? u => { } : v => { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression27", 101, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression27", 101, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression27", 101, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression28 (case 102)
#[test]
fn ternary_expression_28() {
    let src = r#"T ? u => (d ? e => 1 : f => 2)(3) : c => 2"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression28", 102, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression28", 102, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression28", 102, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression30 (case 103)
#[test]
fn ternary_expression_30() {
    let src = r#"T ? a ?"#;
    let expected = Some(ExpectedDiagnostics { count: 5, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression30", 103, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression30", 103, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression30", 103, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression31 (case 104)
#[test]
fn ternary_expression_31() {
    let src = r#"T ? a ="#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression31", 104, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression31", 104, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression31", 104, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression32 (case 105)
#[test]
fn ternary_expression_32() {
    let src = r#"T ? a = b"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression32", 105, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression32", 105, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression32", 105, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression33 (case 106)
#[test]
fn ternary_expression_33() {
    let src = r#"T ? a = b : "#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression33", 106, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression33", 106, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression33", 106, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression34 (case 107)
#[test]
fn ternary_expression_34() {
    let src = r#"T ? m(out c"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression34", 107, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression34", 107, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression34", 107, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression35 (case 108)
#[test]
fn ternary_expression_35() {
    let src = r#"T ? m(ref c"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression35", 108, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression35", 108, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression35", 108, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression36 (case 109)
#[test]
fn ternary_expression_36() {
    let src = r#"T ? m(ref out"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression36", 109, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression36", 109, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression36", 109, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression37 (case 110)
#[test]
fn ternary_expression_37() {
    let src = r#"T ? m(ref out c"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression37", 110, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression37", 110, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression37", 110, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression38 (case 111)
#[test]
fn ternary_expression_38() {
    let src = r#"T ? m(this"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression38", 111, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression38", 111, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression38", 111, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression39 (case 112)
#[test]
fn ternary_expression_39() {
    let src = r#"T ? m(this."#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression39", 112, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression39", 112, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression39", 112, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression40 (case 113)
#[test]
fn ternary_expression_40() {
    let src = r#"T ? m(this<"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression40", 113, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression40", 113, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression40", 113, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression41 (case 114)
#[test]
fn ternary_expression_41() {
    let src = r#"T ? m(this["#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression41", 114, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression41", 114, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression41", 114, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression41A (case 115)
#[test]
fn ternary_expression_41_a() {
    let src = r#"T ? m(this a"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression41A", 115, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression41A", 115, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression41A", 115, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression42 (case 116)
#[test]
fn ternary_expression_42() {
    let src = r#"T ? m(this("#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression42", 116, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression42", 116, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression42", 116, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression43 (case 117)
#[test]
fn ternary_expression_43() {
    let src = r#"T ? m(T["#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression43", 117, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression43", 117, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression43", 117, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression44 (case 118)
#[test]
fn ternary_expression_44() {
    let src = r#"T ? m(T[1"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression44", 118, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression44", 118, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression44", 118, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression45 (case 119)
#[test]
fn ternary_expression_45() {
    let src = r#"T ? m(T[1]"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression45", 119, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression45", 119, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression45", 119, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_MethodDecl46 (case 120)
#[test]
fn ternary_method_decl_46() {
    let src = r#"T ? a(T ? a ="#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl46", 120, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl46", 120, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_MethodDecl46", 120, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression47 (case 121)
#[test]
fn ternary_expression_47() {
    let src = r#"T ? a(T)"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression47", 121, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression47", 121, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression47", 121, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression48 (case 122)
#[test]
fn ternary_expression_48() {
    let src = r#"T ? a(ref int.MaxValue)"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression48", 122, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression48", 122, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression48", 122, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression49 (case 123)
#[test]
fn ternary_expression_49() {
    let src = r#"T ? a(ref a,"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression49", 123, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression49", 123, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression49", 123, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression50 (case 124)
#[test]
fn ternary_expression_50() {
    let src = r#"T ? a(,"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression50", 124, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression50", 124, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression50", 124, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression51 (case 125)
#[test]
fn ternary_expression_51() {
    let src = r#"T ? a(T ? b[1] : b[2])"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression51", 125, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression51", 125, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression51", 125, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression52 (case 126)
#[test]
fn ternary_expression_52() {
    let src = r#"
T ? f(a ? b : c)
"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression52", 126, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression52", 126, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression52", 126, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_Expression_GenericAmbiguity1 (case 127)
#[test]
fn ternary_expression_generic_ambiguity_1() {
    let src = r#"T ? m(a < b, c > d) :"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression_GenericAmbiguity1", 127, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression_GenericAmbiguity1", 127, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_Expression_GenericAmbiguity1", 127, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_WithQuery_FieldDecl1 (case 128)
#[test]
fn ternary_with_query_field_decl_1() {
    let src = r#"
T? from;
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_FieldDecl1", 128, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_FieldDecl1", 128, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_FieldDecl1", 128, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_WithQuery_Expression1 (case 129)
#[test]
fn ternary_with_query_expression_1() {
    let src = r#"
T ? from
"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression1", 129, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression1", 129, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression1", 129, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_WithQuery_Expression2 (case 130)
#[test]
fn ternary_with_query_expression_2() {
    let src = r#"
T ? from x
"#;
    let expected = Some(ExpectedDiagnostics { count: 6, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression2", 130, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression2", 130, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression2", 130, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_WithQuery_Expression3 (case 131)
#[test]
fn ternary_with_query_expression_3() {
    let src = r#"
T ? f(from
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression3", 131, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression3", 131, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression3", 131, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.Ternary_WithQuery_Expression4 (case 132)
#[test]
fn ternary_with_query_expression_4() {
    let src = r#"
T ? f(from x
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression4", 132, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression4", 132, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "Ternary_WithQuery_Expression4", 132, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_Identifier (case 133)
#[test]
fn from_identifier() {
    let src = r#"from"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Identifier", 133, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Identifier", 133, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Identifier", 133, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_FieldDecl (case 134)
#[test]
fn from_field_decl() {
    let src = r#"from c"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl", 134, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl", 134, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl", 134, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_FieldDecl2 (case 135)
#[test]
fn from_field_decl_2() {
    let src = r#"from x,"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl2", 135, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl2", 135, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl2", 135, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_FieldDecl3 (case 136)
#[test]
fn from_field_decl_3() {
    let src = r#"from x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl3", 136, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl3", 136, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl3", 136, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_FieldDecl4 (case 137)
#[test]
fn from_field_decl_4() {
    let src = r#"from x ="#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl4", 137, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl4", 137, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl4", 137, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_FieldDecl5 (case 138)
#[test]
fn from_field_decl_5() {
    let src = r#"from x["#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl5", 138, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl5", 138, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_FieldDecl5", 138, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_MethodDecl1 (case 139)
#[test]
fn from_method_decl_1() {
    let src = r#"from c("#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl1", 139, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl1", 139, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl1", 139, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_MethodDecl2 (case 140)
#[test]
fn from_method_decl_2() {
    let src = r#"from a<"#;
    let expected = Some(ExpectedDiagnostics { count: 5, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl2", 140, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl2", 140, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl2", 140, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_MethodDecl3 (case 141)
#[test]
fn from_method_decl_3() {
    let src = r#"from a."#;
    let expected = Some(ExpectedDiagnostics { count: 4, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl3", 141, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl3", 141, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl3", 141, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_MethodDecl4 (case 142)
#[test]
fn from_method_decl_4() {
    let src = r#"from a::"#;
    let expected = Some(ExpectedDiagnostics { count: 5, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl4", 142, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl4", 142, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl4", 142, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_MethodDecl5 (case 143)
#[test]
fn from_method_decl_5() {
    let src = r#"from global::"#;
    let expected = Some(ExpectedDiagnostics { count: 5, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl5", 143, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl5", 143, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_MethodDecl5", 143, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_PropertyDecl1 (case 144)
#[test]
fn from_property_decl_1() {
    let src = r#"from c {"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_PropertyDecl1", 144, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_PropertyDecl1", 144, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_PropertyDecl1", 144, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_Query1 (case 145)
#[test]
fn from_query_1() {
    let src = r#"from c d"#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query1", 145, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query1", 145, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query1", 145, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_Query2 (case 146)
#[test]
fn from_query_2() {
    let src = r#"from x* a"#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query2", 146, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query2", 146, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query2", 146, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_Query3 (case 147)
#[test]
fn from_query_3() {
    let src = r#"from x? a"#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query3", 147, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query3", 147, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query3", 147, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_Query4 (case 148)
#[test]
fn from_query_4() {
    let src = r#"from x[] a"#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query4", 148, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query4", 148, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query4", 148, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_Query5 (case 149)
#[test]
fn from_query_5() {
    let src = r#"from goo in"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query5", 149, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query5", 149, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query5", 149, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.From_Query6 (case 150)
#[test]
fn from_query_6() {
    let src = r#"from goo.bar in"#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query6", 150, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query6", 150, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "From_Query6", 150, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.GlobalStatementSeparators_Comma1 (case 151)
#[test]
fn global_statement_separators_comma_1() {
    let src = r#"a < b,c."#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_Comma1", 151, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_Comma1", 151, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_Comma1", 151, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.GlobalStatementSeparators_Comma2 (case 152)
#[test]
fn global_statement_separators_comma_2() {
    let src = r#"
a < b,
void goo() { }
"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_Comma2", 152, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_Comma2", 152, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_Comma2", 152, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.GlobalStatementSeparators_ClosingParen (case 153)
#[test]
fn global_statement_separators_closing_paren() {
    let src = r#"
a < b)
void goo() { }
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_ClosingParen", 153, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_ClosingParen", 153, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_ClosingParen", 153, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.GlobalStatementSeparators_ClosingBracket (case 154)
#[test]
fn global_statement_separators_closing_bracket() {
    let src = r#"
a < b]
void goo() { }
"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_ClosingBracket", 154, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_ClosingBracket", 154, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_ClosingBracket", 154, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: ScriptParsingTests.GlobalStatementSeparators_ClosingBrace (case 155)
#[test]
fn global_statement_separators_closing_brace() {
    let src = r#"
a < b}
void goo() { }
"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_ClosingBrace", 155, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_ClosingBrace", 155, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("script_parsing_tests", "ScriptParsingTests", "GlobalStatementSeparators_ClosingBrace", 155, None, CaseData::File { unit: &unit, src, original: None });
    }
}

