use crate::generator::model::{ExpectedDiagnostics, ExtractedTest, TestCategory};

#[allow(dead_code)]
pub fn emit_tests_for_group(
    module_name: &str,
    source_stem: &str,
    tests: &[ExtractedTest],
) -> anyhow::Result<String> {
    use std::fmt::Write as _;

    let mut out = String::new();
    // Header
    writeln!(out, "// Auto-generated from Roslyn: {}", source_stem).ok();

    // Determine which imports are needed
    let has_stmt = tests
        .iter()
        .any(|t| matches!(t.category, TestCategory::Statement));
    let has_non_stmt = tests.iter().any(|t| {
        matches!(
            t.category,
            TestCategory::Tree | TestCategory::Declaration | TestCategory::Expression
        )
    });

    // Determine if any test carries diagnostics expectations
    let any_diag = tests.iter().any(|t| match t.expected.as_ref() {
        Some(e) => e.count > 0 || !e.items.is_empty(),
        None => false,
    });

    // Common imports (always used)
    writeln!(out, "use bsharp_parser::syntax::span::Span;").ok();
    writeln!(out, "use crate::custom_asserts::after_parse;").ok();
    writeln!(out, "use crate::custom_asserts::after_parse::CaseData;").ok();

    // Conditional imports
    if has_non_stmt {
        writeln!(out, "use bsharp_parser::bsharp::parse_csharp_source_strict;").ok();
    }

fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s,
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false" | "fn" | "for"
            | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return"
            | "Self" | "self" | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use"
            | "where" | "while" | "async" | "await" | "dyn" | "abstract" | "become" | "box" | "do" | "final"
            | "macro" | "override" | "priv" | "typeof" | "unsized" | "virtual" | "yield"
    )
}

fn sanitize_rust_ident(s: &str) -> String {
    if is_rust_keyword(s) {
        format!("{}_", s)
    } else {
        s.to_string()
    }
}
    if has_stmt {
        writeln!(out, "use bsharp_parser::statement_parser::parse_statement_ws;").ok();
    }
    if any_diag {
        writeln!(
            out,
            "use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;"
        )
        .ok();
    }

    // Prepare per-method occurrence counting to avoid duplicate names
    use std::collections::HashMap;
    let mut occurrences: HashMap<String, usize> = HashMap::new();

    for (idx, t) in tests.iter().enumerate() {
        let idx1 = idx + 1;
        let base = if let Some(m) = &t.method_name {
            let stripped = strip_leading_test(m);
            let snake = to_snake_case(stripped);
            let snake = if snake.is_empty() { format!("case_{}", idx1) } else { snake };
            sanitize_rust_ident(&snake)
        } else {
            sanitize_rust_ident(&format!("case_{}", idx1))
        };
        let entry = occurrences.entry(base.clone()).or_insert(0);
        *entry += 1;
        let fn_name = if *entry == 1 { base.clone() } else { sanitize_rust_ident(&format!("{}_case_{}", base, *entry)) };
        let roslyn_method = t.method_name.as_deref().unwrap_or("");

        writeln!(out, "/// Roslyn: {}.{} (case {})", source_stem, roslyn_method, idx1).ok();
        writeln!(out, "#[test]").ok();
        writeln!(out, "fn {}() {{", fn_name).ok();

        // Source literal
        let src_lit = to_rust_raw_string(&t.code);
        writeln!(out, "    let src = {};", src_lit).ok();

        // Diagnostics expectation per case
        let (has_expected, expected_str) = match t.expected.as_ref() {
            Some(e) if e.count > 0 || !e.items.is_empty() => {
                // For now we only serialize count and leave items empty
                let count = e.count.max(e.items.len());
                if any_diag {
                    (
                        true,
                        format!(
                            "    let expected = Some(ExpectedDiagnostics {{ count: {}, items: vec![] }});",
                            count
                        ),
                    )
                } else {
                    (true, String::new())
                }
            }
            _ => (false, String::new()),
        };
        if any_diag {
            if has_expected {
                writeln!(out, "{}", expected_str).ok();
            } else {
                writeln!(out, "    let expected: Option<ExpectedDiagnostics> = None; ").ok();
            }
        }

        writeln!(out, "    let span = Span::new(src);").ok();

        match t.category {
            TestCategory::Tree | TestCategory::Declaration | TestCategory::Expression | TestCategory::Name | TestCategory::Type | TestCategory::ParameterList | TestCategory::AttributeList => {
                match t.category {
                    TestCategory::Tree => {
                        writeln!(out, "    let r = parse_csharp_source_strict(span);").ok();
                    }
                    TestCategory::Declaration => {
                        let wrapped = wrap_declaration(&t.code);
                        let lit = to_rust_raw_string(&wrapped);
                        writeln!(out, "    let src2 = {};", lit).ok();
                        writeln!(out, "    let span2 = Span::new(src2);").ok();
                        writeln!(out, "    let r = parse_csharp_source_strict(span2);").ok();
                    }
                    TestCategory::Expression => {
                        let wrapped = wrap_expression(&t.code);
                        let lit = to_rust_raw_string(&wrapped);
                        writeln!(out, "    let src2 = {};", lit).ok();
                        writeln!(out, "    let span2 = Span::new(src2);").ok();
                        writeln!(out, "    let r = parse_csharp_source_strict(span2);").ok();
                    }
                    TestCategory::Name => {
                        let wrapped = wrap_name(&t.code);
                        let lit = to_rust_raw_string(&wrapped);
                        writeln!(out, "    let src2 = {};", lit).ok();
                        writeln!(out, "    let span2 = Span::new(src2);").ok();
                        writeln!(out, "    let r = parse_csharp_source_strict(span2);").ok();
                    }
                    TestCategory::Type => {
                        let wrapped = wrap_type(&t.code);
                        let lit = to_rust_raw_string(&wrapped);
                        writeln!(out, "    let src2 = {};", lit).ok();
                        writeln!(out, "    let span2 = Span::new(src2);").ok();
                        writeln!(out, "    let r = parse_csharp_source_strict(span2);").ok();
                    }
                    TestCategory::ParameterList => {
                        let wrapped = wrap_parameter_list(&t.code);
                        let lit = to_rust_raw_string(&wrapped);
                        writeln!(out, "    let src2 = {};", lit).ok();
                        writeln!(out, "    let span2 = Span::new(src2);").ok();
                        writeln!(out, "    let r = parse_csharp_source_strict(span2);").ok();
                    }
                    TestCategory::AttributeList => {
                        let wrapped = wrap_attribute_list(&t.code);
                        let lit = to_rust_raw_string(&wrapped);
                        writeln!(out, "    let src2 = {};", lit).ok();
                        writeln!(out, "    let span2 = Span::new(src2);").ok();
                        writeln!(out, "    let r = parse_csharp_source_strict(span2);").ok();
                    }
                    _ => {}
                }

                if any_diag {
                    writeln!(out, "{}", "    if let Some(expected) = expected {").ok();
                    writeln!(out, "{}", "        match r {").ok();
                    writeln!(out, "{}", "            Ok((_rest, unit)) => {").ok();
                    if matches!(t.category, TestCategory::Tree) {
                        writeln!(out, "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, Some(expected.clone()), CaseData::File {{ unit: &unit, src, original: None }});",
                                module_name, source_stem, roslyn_method, idx1).ok();
                    } else {
                        writeln!(out, "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, Some(expected.clone()), CaseData::File {{ unit: &unit, src: src2, original: Some(src) }});",
                                module_name, source_stem, roslyn_method, idx1).ok();
                    }
                    writeln!(out, "            }}").ok();
                    writeln!(out, "{}", "            Err(_) => {").ok();
                    writeln!(out, "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, Some(expected.clone()), CaseData::Empty);",
                            module_name, source_stem, roslyn_method, idx1).ok();
                    writeln!(out, "            }}").ok();
                    writeln!(out, "        }}").ok();
                    writeln!(out, "{}", "    } else {").ok();
                    writeln!(out, "        assert!(r.is_ok(), \"parse failed: {{:?}}\", r.err());").ok();
                    writeln!(out, "        let (_rest, unit) = r.unwrap();").ok();
                    if matches!(t.category, TestCategory::Tree) {
                        writeln!(out, "        after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, None, CaseData::File {{ unit: &unit, src, original: None }});",
                                module_name, source_stem, roslyn_method, idx1).ok();
                    } else {
                        writeln!(out, "        after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, None, CaseData::File {{ unit: &unit, src: src2, original: Some(src) }});",
                                module_name, source_stem, roslyn_method, idx1).ok();
                    }
                    writeln!(out, "    }}").ok();
                } else {
                    // No diagnostics import; use None directly to avoid type reference
                    writeln!(out, "{}", "    match r {").ok();
                    writeln!(out, "{}", "        Ok((_rest, unit)) => {").ok();
                    if matches!(t.category, TestCategory::Tree) {
                        writeln!(out, "            after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, None, CaseData::File {{ unit: &unit, src, original: None }});",
                                module_name, source_stem, roslyn_method, idx1).ok();
                    } else {
                        writeln!(out, "            after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, None, CaseData::File {{ unit: &unit, src: src2, original: Some(src) }});",
                                module_name, source_stem, roslyn_method, idx1).ok();
                    }
                    writeln!(out, "        }}").ok();
                    writeln!(out, "        Err(e) => panic!(\"parse failed: {{:?}}\", e),").ok();
                    writeln!(out, "    }}").ok();
                }
            }
            TestCategory::Statement => {
                writeln!(out, "    let r = parse_statement_ws(span);").ok();
                if any_diag {
                    writeln!(out, "{}", "    if let Some(expected) = expected {").ok();
                    writeln!(out, "{}", "        match r {").ok();
                    writeln!(out, "{}", "            Ok((rest, ast)) => {").ok();
                    writeln!(out, "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, Some(expected.clone()), CaseData::Statement {{ ast: &ast, src }});",
                            module_name, source_stem, roslyn_method, idx1).ok();
                    writeln!(out, "            }}").ok();
                    writeln!(out, "            Err(_) => {{").ok();
                    writeln!(out, "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, Some(expected.clone()), CaseData::Empty);",
                            module_name, source_stem, roslyn_method, idx1).ok();
                    writeln!(out, "            }}").ok();
                    writeln!(out, "        }}").ok();
                    writeln!(out, "    }} else {{").ok();
                    writeln!(out, "        assert!(r.is_ok(), \"parse failed: {{:?}}\", r.err());").ok();
                    writeln!(out, "        let (rest, ast) = r.unwrap();").ok();
                    writeln!(out, "        assert!(rest.fragment().trim().is_empty(), \"Unconsumed input: {{}}\", rest.fragment());").ok();
                    writeln!(out, "        after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, None, CaseData::Statement {{ ast: &ast, src }});",
                            module_name, source_stem, roslyn_method, idx1).ok();
                    writeln!(out, "    }}").ok();
                } else {
                    writeln!(out, "{}", "    match r {").ok();
                    writeln!(out, "{}", "        Ok((rest, ast)) => {").ok();
                    writeln!(out, "            assert!(rest.fragment().trim().is_empty(), \"Unconsumed input: {{}}\", rest.fragment());").ok();
                    writeln!(out, "            after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, None, CaseData::Statement {{ ast: &ast, src }});",
                            module_name, source_stem, roslyn_method, idx1).ok();
                    writeln!(out, "        }}").ok();
                    writeln!(out, "        Err(e) => panic!(\"parse failed: {{:?}}\", e),").ok();
                    writeln!(out, "    }}").ok();
                }
            }
        }

        writeln!(out, "}}\n").ok();
    }

    Ok(out)
}

fn wrap_declaration(code: &str) -> String {
    let trimmed = code.trim_start();
    let is_top = trimmed.starts_with("class ")
        || trimmed.starts_with("struct ")
        || trimmed.starts_with("interface ")
        || trimmed.starts_with("record ")
        || trimmed.starts_with("enum ")
        || trimmed.starts_with("namespace ");
    if is_top {
        code.to_string()
    } else {
        format!("class C {{ {} }}", code)
    }
}

fn wrap_expression(code: &str) -> String {
    format!("class C {{ void M() {{ {}; }} }}", code)
}

fn wrap_name(code: &str) -> String {
    format!("class C {{ {} f; }}", code)
}

fn wrap_type(code: &str) -> String {
    format!("class C {{ {} f; }}", code)
}

fn wrap_parameter_list(code: &str) -> String {
    format!("class C {{ void M{} {{ }} }}", code)
}

fn wrap_attribute_list(code: &str) -> String {
    format!("class C {{ {} void M() {{ }} }}", code)
}

fn to_rust_raw_string(s: &str) -> String {
    // Choose a number of # that avoids collisions with content
    let mut hashes = 1usize;
    loop {
        let delim = "#".repeat(hashes);
        let open = format!("r{}\"", delim);
        let close = format!("\"{}", delim);
        if !s.contains(&open) && !s.contains(&close) {
            break;
        }
        hashes += 1;
        if hashes > 10 {
            break;
        }
    }
    let delim = "#".repeat(hashes);
    format!("r{d}\"{s}\"{d}", d = delim, s = s)
}

fn to_snake_case(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    #[derive(Copy, Clone, PartialEq)]
    enum Cat {
        None,
        Upper,
        Lower,
        Digit,
    }
    let mut out = String::with_capacity(s.len() + 8);
    let mut prev: Cat = Cat::None;
    for ch in s.chars() {
        let cat = if ch.is_ascii_uppercase() {
            Cat::Upper
        } else if ch.is_ascii_lowercase() {
            Cat::Lower
        } else if ch.is_ascii_digit() {
            Cat::Digit
        } else {
            Cat::None
        };
        if cat == Cat::None {
            if !out.ends_with('_') {
                out.push('_');
            }
            prev = Cat::None;
            continue;
        }
        let boundary = matches!(
            (prev, cat),
            (Cat::Lower, Cat::Upper)
                | (Cat::Lower, Cat::Digit)
                | (Cat::Upper, Cat::Digit)
                | (Cat::Digit, Cat::Lower)
                | (Cat::Digit, Cat::Upper)
        );
        if boundary && !out.ends_with('_') {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
        prev = cat;
    }
    while out.ends_with('_') {
        out.pop();
    }
    if out
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        out.insert(0, '_');
    }
    if out.is_empty() {
        out.push_str("case");
    }
    out
}

fn strip_leading_test(name: &str) -> &str {
    if name.to_ascii_lowercase().starts_with("test") {
        &name[4..]
    } else {
        name
    }
}
