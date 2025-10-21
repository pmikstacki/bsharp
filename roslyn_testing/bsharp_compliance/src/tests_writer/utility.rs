use std::path::PathBuf;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use regex::Regex;
use crate::tests_writer::codegen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category { Tree, Statement, Declaration, Expression }

// (removed unused helper sanitize_ident)

pub fn to_snake_case(s: &str) -> String {
    if s.is_empty() { return String::new(); }
    let mut out = String::with_capacity(s.len() + 8);
    #[derive(Copy, Clone, PartialEq)]
    enum Cat { None, Upper, Lower, Digit }
    let mut prev: Cat = Cat::None;
    for ch in s.chars() {
        let cat = if ch.is_ascii_uppercase() { Cat::Upper }
            else if ch.is_ascii_lowercase() { Cat::Lower }
            else if ch.is_ascii_digit() { Cat::Digit }
            else { Cat::None };
        if cat == Cat::None {
            if !out.ends_with('_') { out.push('_'); }
            prev = Cat::None;
            continue;
        }
        // Insert '_' between boundaries:
        // - lower/digit -> upper (CamelCase)
        // - alpha -> digit
        // - digit -> alpha
        let boundary = matches!((prev, cat),
            (Cat::Lower, Cat::Upper)
            | (Cat::Lower, Cat::Digit)
            | (Cat::Upper, Cat::Digit)
            | (Cat::Digit, Cat::Lower)
            | (Cat::Digit, Cat::Upper)
        );
        if boundary && !out.ends_with('_') { out.push('_'); }
        out.push(ch.to_ascii_lowercase());
        prev = cat;
    }
    while out.ends_with('_') { out.pop(); }
    if out.starts_with(|c: char| c.is_ascii_digit()) { out.insert(0, '_'); }
    if out.is_empty() { out.push_str("case"); }
    out
}

pub fn strip_leading_test(name: &str) -> &str {
    let lower = name.as_bytes();
    if lower.len() >= 4 && lower[0..4].eq_ignore_ascii_case(b"test") {
        &name[4..]
    } else {
        name
    }
}

pub fn collect_test_methods(content: &str) -> Vec<(usize, String)> {
    // Rough detection of test method declarations following attributes.
    // We do not attempt full C# parsing; this is good enough for mapping.
    let re = Regex::new(r"(?m)^[ \t]*(?:\[[^\]]+\][ \t]*\n[ \t]*)*(?:public|private|protected|internal)\b[^{\n]*?\b([A-Za-z_][A-Za-z0-9_]*)\s*\(").unwrap();
    let mut v = Vec::new();
    for cap in re.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            v.push((m.start(), m.as_str().to_string()));
        }
    }
    v.sort_by_key(|(i, _)| *i);
    v
}

pub fn find_enclosing_method_name(methods: &[(usize, String)], pos: usize) -> Option<String> {
    if methods.is_empty() { return None; }
    let mut last: Option<String> = None;
    for (i, name) in methods.iter() {
        if *i <= pos { last = Some(name.clone()); } else { break; }
    }
    last
}

pub fn prevalidate(t: &ExtractedTest) -> bool {
    match t.category {
        Category::Tree => parse_csharp_source_strict(Span::new(t.code.as_str())).is_ok(),
        Category::Statement => match parse_statement_ws(Span::new(t.code.as_str())) {
            Ok((rest, _)) => rest.fragment().trim().is_empty(),
            Err(_) => false,
        },
        Category::Declaration => {
            let wrapped = codegen::wrap_declaration(&t.code);
            parse_csharp_source_strict(Span::new(wrapped.as_str())).is_ok()
        }
        Category::Expression => {
            let wrapped = codegen::wrap_expression(&t.code);
            parse_csharp_source_strict(Span::new(wrapped.as_str())).is_ok()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtractedTest {
    pub category: Category,
    pub method_name: Option<String>,
    pub code: String,
    pub expected_diag_count: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub max_per_file: usize,
    pub skip_overrides: bool,
    pub skip_diagnostics: bool,
    pub use_new_emitter: bool,
    pub prevalidate: bool,
    pub structure_mode: bool,
}