#![cfg(test)]

use parser::trivia::preprocessor_directive_parser::parse_preprocessor_directive;
use syntax::trivia::preprocessor::PreprocessorDirective;

#[test]
fn test_parse_pragma_warning_disable() {
    let code = "#pragma warning disable CS0168";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Pragma { pragma } => {
            assert_eq!(pragma, "warning disable CS0168");
        }
        _ => panic!("Expected Pragma directive"),
    }
}

#[test]
fn test_parse_pragma_warning_restore() {
    let code = "#pragma warning restore CS0219";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Pragma { pragma } => {
            assert_eq!(pragma, "warning restore CS0219");
        }
        _ => panic!("Expected Pragma directive"),
    }
}

#[test]
fn test_parse_pragma_nullable_enable() {
    let code = "#pragma nullable enable";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Pragma { pragma } => {
            assert_eq!(pragma, "nullable enable");
        }
        _ => panic!("Expected Pragma directive"),
    }
}

#[test]
fn test_parse_pragma_checksum() {
    let code = r#"#pragma checksum "file.cs" "{12345678-1234}" "ABC""#;
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Pragma { pragma } => {
            assert!(pragma.starts_with("checksum"));
        }
        _ => panic!("Expected Pragma directive"),
    }
}

#[test]
fn test_parse_pragma_empty() {
    let code = "#pragma";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Pragma { pragma } => {
            assert_eq!(pragma, "");
        }
        _ => panic!("Expected Pragma directive"),
    }
}

#[test]
fn test_parse_line_number() {
    let code = "#line 100";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Line { line } => {
            assert_eq!(line, "100");
        }
        _ => panic!("Expected Line directive"),
    }
}

#[test]
fn test_parse_line_number_and_file() {
    let code = r#"#line 200 "source.cs""#;
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Line { line } => {
            assert_eq!(line, r#"200 "source.cs""#);
        }
        _ => panic!("Expected Line directive"),
    }
}

#[test]
fn test_parse_line_default() {
    let code = "#line default";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Line { line } => {
            assert_eq!(line, "default");
        }
        _ => panic!("Expected Line directive"),
    }
}

#[test]
fn test_parse_line_hidden() {
    let code = "#line hidden";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Line { line } => {
            assert_eq!(line, "hidden");
        }
        _ => panic!("Expected Line directive"),
    }
}

#[test]
fn test_parse_line_empty() {
    let code = "#line";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Line { line } => {
            assert_eq!(line, "");
        }
        _ => panic!("Expected Line directive"),
    }
}

#[test]
fn test_parse_pragma_with_whitespace() {
    let code = "#pragma   warning   disable   CS0168  ";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Pragma { pragma } => {
            assert_eq!(pragma, "warning   disable   CS0168");
        }
        _ => panic!("Expected Pragma directive"),
    }
}

#[test]
fn test_parse_line_with_whitespace() {
    let code = "#line   500   ";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Line { line } => {
            assert_eq!(line, "500");
        }
        _ => panic!("Expected Line directive"),
    }
}

#[test]
fn test_directive_consumes_until_newline() {
    let code = "#pragma warning disable\nclass Test {}";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (remaining, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Pragma { pragma } => {
            assert_eq!(pragma, "warning disable");
        }
        _ => panic!("Expected Pragma directive"),
    }
    // Should have stopped at newline
    assert!(remaining.trim_start().starts_with("class"));
}

#[test]
fn test_line_directive_with_span_info() {
    let code = r#"#line (10,5)-(10,30) "file.cs""#;
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Line { line } => {
            assert!(line.contains("(10,5)"));
        }
        _ => panic!("Expected Line directive"),
    }
}

#[test]
fn test_unknown_region_directive() {
    let code = "#region My Region";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Region { name } => {
            assert_eq!(name, Some("My Region".to_string()));
        }
        _ => panic!("Expected Region directive for #region"),
    }
}

#[test]
fn test_unknown_define_directive() {
    let code = "#define FOO";
    let result = parse_preprocessor_directive(code.into());
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();
    match directive {
        PreprocessorDirective::Define { symbol } => {
            assert_eq!(symbol.to_string(), "FOO");
        }
        _ => panic!("Expected Define directive for #define"),
    }
}
