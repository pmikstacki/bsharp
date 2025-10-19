// Integration tests for parsing preprocessor directives

use parser::trivia::preprocessor_directive_parser::parse_preprocessor_directive;
use syntax::identifier::Identifier;
use syntax::trivia::preprocessor::PreprocessorDirective;

#[test]
fn test_parse_define() {
    let code = "#define DEBUG";
    let (rest, dir) = parse_preprocessor_directive(code.into()).expect("should parse #define");
    assert!(rest.fragment().is_empty());
    match dir {
        PreprocessorDirective::Define { symbol } => assert_eq!(symbol.to_string(), "DEBUG"),
        other => panic!("expected Define, got {:?}", other),
    }
}

#[test]
fn test_parse_undef() {
    let code = "#undef RELEASE";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #undef");
    match dir {
        PreprocessorDirective::Undef { symbol } => assert_eq!(symbol.to_string(), "RELEASE"),
        other => panic!("expected Undef, got {:?}", other),
    }
}

#[test]
fn test_parse_if() {
    let code = "#if SYMBOL";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #if");
    assert!(matches!(dir, PreprocessorDirective::If { condition } if condition == "SYMBOL"));
}

#[test]
fn test_parse_elif() {
    let code = "#elif OTHER_SYMBOL";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #elif");
    assert!(
        matches!(dir, PreprocessorDirective::Elif { condition } if condition == "OTHER_SYMBOL")
    );
}

#[test]
fn test_parse_else() {
    let code = "#else";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #else");
    assert!(matches!(dir, PreprocessorDirective::Else));
}

#[test]
fn test_parse_endif() {
    let code = "#endif";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #endif");
    assert!(matches!(dir, PreprocessorDirective::Endif));
}

#[test]
fn test_parse_region() {
    let code = "#region MyRegion";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #region");
    assert!(
        matches!(dir, PreprocessorDirective::Region { name } if name == Some("MyRegion".to_string()))
    );
}

#[test]
fn test_parse_region_no_name() {
    let code = "#region";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #region without name");
    assert!(matches!(dir, PreprocessorDirective::Region { name } if name.is_none()));
}

#[test]
fn test_parse_endregion() {
    let code = "#endregion";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #endregion");
    assert!(matches!(dir, PreprocessorDirective::EndRegion));
}

#[test]
fn test_parse_error() {
    let code = "#error This is an error message";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #error");
    assert!(
        matches!(dir, PreprocessorDirective::Error { message } if message == "This is an error message")
    );
}

#[test]
fn test_parse_warning() {
    let code = "#warning This is a warning message";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #warning");
    assert!(
        matches!(dir, PreprocessorDirective::Warning { message } if message == "This is a warning message")
    );
}

#[test]
fn test_parse_pragma_simple() {
    let code = "#pragma warning disable CS0168";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #pragma");
    assert!(
        matches!(dir, PreprocessorDirective::Pragma { pragma } if pragma == "warning disable CS0168")
    );
}

#[test]
fn test_parse_line_simple() {
    let code = "#line 100";
    let (_, dir) = parse_preprocessor_directive(code.into()).expect("should parse #line");
    assert!(matches!(dir, PreprocessorDirective::Line { line } if line == "100"));
}
