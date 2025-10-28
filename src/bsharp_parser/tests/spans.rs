use bsharp_parser::syntax::span::Span;

#[test]
fn ident_spanned_excludes_trivia() {
    let src = "  foo ";
    let input = Span::new(src);
    let (_rest, s) = bsharp_parser::parser::identifier_parser::parse_identifier_spanned(input).unwrap();

    assert_eq!(s.abs.start, 2);
    assert_eq!(s.abs.end, 5);
    assert_eq!(s.rel.start.line, 1);
    assert_eq!(s.rel.start.offset, 2);
    assert_eq!(s.rel.end.line, 1);
    assert_eq!(s.rel.end.offset, 5);
    assert_eq!(s.abs.slice(src), "foo");
}

#[test]
fn using_directive_spanned_excludes_trivia() {
    use bsharp_parser::Parsable;
    let src = "  using System;  ";
    let input = Span::new(src);
    let (_rest, s) = bsharp_parser::syntax::declarations::UsingDirective::parse(input).unwrap();
    assert_eq!(s.abs.start, 2);
    assert!(s.abs.end <= src.len() - 2);
    assert!(s.abs.slice(src).starts_with("using "));
}

#[test]
fn expression_spanned_trivia_excluded() {
    let src = "  1 + 2  ";
    let input = Span::new(src);
    let (_rest, s) = bsharp_parser::parser::expressions::primary_expression_parser::parse_expression_spanned(input).unwrap();
    // expression core starts at first non-ws
    assert_eq!(s.abs.start, 2);
    assert!(s.abs.end <= src.len());
    assert_eq!(s.rel.start.line, 1);
    assert_eq!(s.rel.start.offset, 2);
}

#[test]
fn statement_spanned_return_excludes_trivia() {
    let src = "\n   return 42;  ";
    let input = Span::new(src);
    let (_rest, s) = bsharp_parser::parser::statement_parser::parse_statement_ws_spanned(input).unwrap();
    // After initial newline, span starts at first non-ws of 'return'
    // newline is 1 byte, then three spaces -> offset 4
    assert_eq!(s.abs.start, 4);
    assert!(s.abs.end > s.abs.start);
    assert_eq!(s.rel.start.line, 2);
    assert_eq!(s.rel.start.offset, 3);
}

#[test]
fn string_spanned_utf8_and_escapes() {
    let src = "  \"hé\"  ";
    let input = Span::new(src);
    let (_rest, s) = bsharp_parser::parser::expressions::literal_parser::parse_literal_spanned(input).unwrap();
    // Starts after two spaces, covers the full quoted string
    assert_eq!(s.abs.start, 2);
    assert_eq!(s.abs.slice(src).chars().next().unwrap(), '"');
}

#[test]
fn raw_string_trimming_multiline_span() {
    let src = "  \"\"\"\n    abc\n    def\n    \"\"\"  ";
    let input = Span::new(src);
    let (_rest, s) = bsharp_parser::parser::expressions::literal_parser::parse_literal_spanned(input).unwrap();
    // Span should capture the full raw string including delimiters, excluding external ws
    assert_eq!(s.abs.start, 2);
    assert!(s.abs.end <= src.len() - 2);
}

#[test]
fn literal_spanned_basic_number() {
    let src = "  12345";
    let input = Span::new(src);
    let (_rest, s) = bsharp_parser::parser::expressions::literal_parser::parse_literal_spanned(input).unwrap();

    // Leading two spaces skipped; number has 5 digits
    assert_eq!(s.abs.start, 2);
    assert_eq!(s.abs.end, 7);
    assert_eq!(s.rel.start.line, 1);
    assert_eq!(s.rel.start.offset, 2);
    assert_eq!(s.rel.end.offset, 7);
    assert_eq!(s.abs.slice(src), "12345");
}

#[test]
fn primary_expression_spanned_this_keyword() {
    let src = "   this";
    let input = Span::new(src);
    let (_rest, s) = bsharp_parser::parser::expressions::primary_expression_parser::parse_primary_expression_spanned(input).unwrap();

    // Three leading spaces excluded; 'this' is 4 bytes
    assert_eq!(s.abs.start, 3);
    assert_eq!(s.abs.end, 7);
    assert_eq!(s.rel.start.line, 1);
    assert_eq!(s.rel.start.offset, 3);
    assert_eq!(s.rel.end.offset, 7);
}
