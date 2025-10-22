use crate::generator::structure_dsl::{ExpectedNode, ExpectedTree, ExtractedStructureTest};
use std::fmt::Write as _;

pub fn emit_structure_tests_for_group(
    _module_name: &str,
    source_stem: &str,
    cases: &[ExtractedStructureTest],
) -> anyhow::Result<String> {
    let mut out = String::new();
    writeln!(
        out,
        "// Auto-generated STRUCTURE tests from Roslyn: {}",
        source_stem
    )
    .ok();
    writeln!(out, "use bsharp_parser::syntax::span::Span;").ok();
    writeln!(
        out,
        "use bsharp_parser::bsharp::parse_csharp_source_strict;"
    )
    .ok();
    writeln!(out, "use crate::custom_asserts::structure_assert;").ok();

    let mut occurrences: std::collections::HashMap<String, usize> = Default::default();
    for (idx, c) in cases.iter().enumerate() {
        let idx1 = idx + 1;
        let base = if let Some(m) = &c.method_name {
            to_snake_case(strip_leading_test(m))
        } else {
            format!("case_{}", idx1)
        };
        let base = sanitize_rust_ident(&base);
        let entry = occurrences.entry(base.clone()).or_insert(0);
        *entry += 1;
        let fn_name = if *entry == 1 {
            base.clone()
        } else {
            format!("{}_case_{}", base, *entry)
        };
        let rust_src = to_rust_raw_string(&c.src_code);

        writeln!(out, "#[test]").ok();
        writeln!(out, "fn {}() {{", fn_name).ok();
        writeln!(out, "    let src = {};", rust_src).ok();
        writeln!(out, "    let span = Span::new(src);").ok();
        writeln!(out, "    let r = parse_csharp_source_strict(span);").ok();
        writeln!(out, "    if let Ok((_rest, unit)) = r {{").ok();
        writeln!(
            out,
            "        let expected = {};",
            emit_expected_tree(&c.expected)
        )
        .ok();
        writeln!(
            out,
            "        structure_assert::assert_tree(&expected, &unit);"
        )
        .ok();
        writeln!(out, "    }}").ok();
        writeln!(out, "}}\n").ok();
    }

    Ok(out)
}

fn emit_expected_tree(t: &ExpectedTree) -> String {
    fn emit_node(n: &ExpectedNode, buf: &mut String, indent: usize) {
        let ind = " ".repeat(indent);
        let tv = match &n.token_value {
            Some(s) => format!("Some({})", to_rust_string(s)),
            None => "None".to_string(),
        };
        let _ = write!(
            buf,
            "{}structure_assert::ExpectedNode {{ kind: {}, token_value: {}, children: vec![",
            ind,
            to_rust_string(&n.kind),
            tv
        );
        if !n.children.is_empty() {
            let mut first = true;
            for ch in &n.children {
                if !first {
                    let _ = write!(buf, ", ");
                }
                first = false;
                emit_node(ch, buf, indent + 4);
            }
        }
        let _ = write!(buf, "] }}");
    }

    let mut s = String::new();
    s.push_str("structure_assert::ExpectedTree { root: ");
    emit_node(&t.root, &mut s, 0);
    s.push_str(" }");
    s
}

fn to_rust_string(s: &str) -> String {
    format!(
        "\"{}\".to_string()",
        s.replace('\\', "\\\\").replace('"', "\\\"")
    )
}

fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "Self"
            | "self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

fn sanitize_rust_ident(s: &str) -> String {
    if is_rust_keyword(s) {
        format!("{}_", s)
    } else {
        s.to_string()
    }
}

fn strip_leading_test(name: &str) -> &str {
    if name.to_ascii_lowercase().starts_with("test") {
        &name[4..]
    } else {
        name
    }
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

fn to_rust_raw_string(s: &str) -> String {
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
