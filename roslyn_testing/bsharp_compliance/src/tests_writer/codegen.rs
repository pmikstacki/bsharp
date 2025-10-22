use crate::tests_writer::utility;
use crate::tests_writer::utility::{Category, ExtractedTest};
use globset::{Glob, GlobSetBuilder};
use std::path::{Path, PathBuf};

pub fn build_globs(
    include: &[String],
    exclude: &[String],
) -> anyhow::Result<(globset::GlobSet, globset::GlobSet)> {
    let mut ib = GlobSetBuilder::new();
    if include.is_empty() {
        // default include everything
        ib.add(Glob::new("**/*.cs")?);
    } else {
        for pat in include {
            let has_sep = pat.contains('/') || pat.contains('\\');
            let has_glob = pat.contains('*') || pat.contains('?') || pat.contains('[');
            if !has_sep && !has_glob {
                // Treat bare filename like a basename match anywhere under src
                let expanded = format!("**/{}", pat);
                ib.add(Glob::new(&expanded)?);
            } else {
                ib.add(Glob::new(pat)?);
            }
        }
    }
    let mut eb = GlobSetBuilder::new();
    for pat in exclude {
        eb.add(Glob::new(pat)?);
    }
    Ok((ib.build()?, eb.build()?))
}

// --- Helpers to parse trailing arguments and count Diagnostic( .. ) occurrences ---

fn find_call_closing_paren(s: &str, open_paren_idx: usize) -> Option<usize> {
    let b = s.as_bytes();
    if open_paren_idx >= b.len() || b[open_paren_idx] != b'(' {
        return None;
    }
    let mut i = open_paren_idx;
    let mut depth: i32 = 0;
    while i < b.len() {
        let c = b[i] as char;
        // Skip verbatim strings @"..." first
        if c == '@' && i + 1 < b.len() && b[i + 1] == b'"' {
            if let Some(j) = skip_csharp_verbatim_string(s, i) {
                i = j;
                continue;
            }
        }
        // Skip raw/regular strings starting with '"'
        if c == '"' {
            if let Some(j) = skip_csharp_string(s, i) {
                i = j;
                continue;
            }
        }
        if c == '(' {
            depth += 1;
        }
        if c == ')' {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn skip_csharp_string(s: &str, i: usize) -> Option<usize> {
    let b = s.as_bytes();
    if b.get(i) != Some(&b'"') {
        return None;
    }
    let mut j = i + 1;
    let mut escaped = false;
    while j < b.len() {
        let c = b[j];
        if escaped {
            escaped = false;
            j += 1;
            continue;
        }
        if c == b'\\' {
            escaped = true;
            j += 1;
            continue;
        }
        if c == b'"' {
            return Some(j + 1);
        }
        j += 1;
    }
    None
}

fn skip_csharp_verbatim_string(s: &str, i: usize) -> Option<usize> {
    let b = s.as_bytes();
    if i + 1 >= b.len() || b[i] != b'@' || b[i + 1] != b'"' {
        return None;
    }
    let mut j = i + 2;
    while j < b.len() {
        if b[j] == b'"' {
            if j + 1 < b.len() && b[j + 1] == b'"' {
                j += 2;
                continue;
            }
            return Some(j + 1);
        }
        j += 1;
    }
    None
}

fn is_ident_boundary(ch: Option<u8>) -> bool {
    match ch {
        Some(c) => !(c as char).is_alphanumeric() && c != b'_',
        None => true,
    }
}

fn count_diagnostic_invocations(s: &str) -> usize {
    let needle = b"Diagnostic(";
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut count = 0usize;
    while i + needle.len() <= bytes.len() {
        if &bytes[i..i + needle.len()] == needle {
            let prev = if i == 0 { None } else { Some(bytes[i - 1]) };
            if is_ident_boundary(prev) {
                count += 1;
            }
            i += needle.len();
            continue;
        }
        i += 1;
    }
    count
}

pub fn is_included(
    include: &globset::GlobSet,
    exclude: &globset::GlobSet,
    rel: &std::path::Path,
) -> bool {
    include.is_match(rel) && !exclude.is_match(rel)
}
pub fn file_overrides_parse_context(content: &str) -> bool {
    content.contains("protected override CSharpSyntaxNode ParseNode(")
        || content.contains("protected override SyntaxTree ParseTree(")
}

pub fn extract_tests(
    content: &str,
    methods: &[(usize, String)],
    skip_diagnostics: bool,
) -> Vec<ExtractedTest> {
    let mut out = Vec::new();
    let mut cursor: usize = 0; // always keep at a char boundary
    loop {
        // Find next Using* or Parse* call and take the earliest
        let using_hit = find_using_call(content, cursor);
        let parse_hit = find_parse_call(content, cursor);
        let next = match (using_hit, parse_hit) {
            (Some(u), Some(p)) => {
                let u_pos = u.2; // call_pos
                let p_pos = p.2;
                if u_pos <= p_pos { Some(u) } else { Some(p) }
            }
            (Some(u), None) => Some(u),
            (None, Some(p)) => Some(p),
            (None, None) => None,
        };
        match next {
            Some((cat, start_args, call_pos)) => {
                if let Some((literal, end_idx)) = extract_first_csharp_string(content, start_args) {
                    // Check if there are more args after the string before closing paren
                    let has_more_args = has_following_nonws_comma_before_paren(content, end_idx);
                    if skip_diagnostics && has_more_args {
                        cursor = end_idx + 1;
                        continue;
                    }
                    let method_name = utility::find_enclosing_method_name(methods, call_pos);
                    let mut expected_diag_count: Option<usize> = None;
                    if has_more_args {
                        // Determine the full call range to scan trailing args for Diagnostic( .. ) occurrences
                        let open_paren_idx = start_args.saturating_sub(1);
                        if let Some(close_paren_idx) =
                            find_call_closing_paren(content, open_paren_idx)
                        {
                            let args_slice = &content[end_idx..=close_paren_idx];
                            let count = count_diagnostic_invocations(args_slice);
                            if count > 0 {
                                expected_diag_count = Some(count);
                            }
                        }
                    }
                    out.push(ExtractedTest {
                        category: cat,
                        method_name,
                        code: literal,
                        expected_diag_count,
                    });
                    cursor = end_idx + 1;
                } else {
                    // Could not extract the string literal; advance past '('
                    cursor = start_args + 1;
                }
            }
            None => break,
        }
    }
    out
}

fn find_using_call(s: &str, from: usize) -> Option<(Category, usize, usize)> {
    // Returns (category, index of opening paren + 1 (start of args), call start index)
    let from = next_char_boundary(s, from);
    let hay = &s[from..];
    let mut candidates: Vec<(usize, Category)> = Vec::new();
    for (kw, cat) in [
        ("UsingTree", Category::Tree),
        ("UsingStatement", Category::Statement),
        ("UsingDeclaration", Category::Declaration),
        ("UsingExpression", Category::Expression),
    ] {
        if let Some(pos) = hay.find(kw) {
            // ensure it is a call with '('
            let mut idx = from + pos + kw.len();
            // skip whitespace
            while idx < s.len() && s.as_bytes()[idx].is_ascii_whitespace() {
                idx += 1;
            }
            // handle optional generic type args e.g., UsingTree<T1,T2>
            if idx < s.len() && s.as_bytes()[idx] == b'<' {
                let mut depth = 0i32;
                while idx < s.len() {
                    let ch = s.as_bytes()[idx] as char;
                    if ch == '<' {
                        depth += 1;
                    } else if ch == '>' {
                        depth -= 1;
                        if depth == 0 {
                            idx += 1;
                            break;
                        }
                    }
                    idx += 1;
                }
                // skip trailing whitespace after generics
                while idx < s.len() && s.as_bytes()[idx].is_ascii_whitespace() {
                    idx += 1;
                }
            }
            if idx < s.len() && s.as_bytes()[idx] == b'(' {
                candidates.push((idx + 1, cat));
            }
        }
    }
    if candidates.is_empty() {
        return None;
    }
    candidates.sort_by_key(|(i, _)| *i);
    candidates.into_iter().next().map(|(i, c)| {
        // call start is at i-1 back to the preceding identifier start
        let mut start = i.saturating_sub(1);
        while start > 0 {
            let ch = s.as_bytes()[start - 1] as char;
            if ch.is_alphanumeric() || ch == '_' {
                start -= 1;
            } else {
                break;
            }
        }
        (c, i, start)
    })
}

fn find_parse_call(s: &str, from: usize) -> Option<(Category, usize, usize)> {
    // Support Roslyn helpers: ParseCompilationUnit, ParseTree, ParseStatement, ParseExpression, ParseMemberDeclaration
    let from = next_char_boundary(s, from);
    let hay = &s[from..];
    let patterns: &[(&str, Category)] = &[
        ("ParseCompilationUnit", Category::Tree),
        ("ParseTree", Category::Tree),
        ("ParseStatement", Category::Statement),
        ("ParseExpression", Category::Expression),
        ("ParseMemberDeclaration", Category::Declaration),
    ];
    let mut best: Option<(usize, Category, usize)> = None;
    for (kw, cat) in patterns {
        if let Some(pos) = hay.find(kw) {
            let mut idx = from + pos + kw.len();
            while idx < s.len() && s.as_bytes()[idx].is_ascii_whitespace() {
                idx += 1;
            }
            // optional generic args
            if idx < s.len() && s.as_bytes()[idx] == b'<' {
                let mut depth = 0i32;
                while idx < s.len() {
                    let ch = s.as_bytes()[idx] as char;
                    if ch == '<' {
                        depth += 1;
                    } else if ch == '>' {
                        depth -= 1;
                        if depth == 0 {
                            idx += 1;
                            break;
                        }
                    }
                    idx += 1;
                }
                while idx < s.len() && s.as_bytes()[idx].is_ascii_whitespace() {
                    idx += 1;
                }
            }
            if idx >= s.len() || s.as_bytes()[idx] != b'(' {
                continue;
            }
            // rewind to start of call (identifier/qualifier)
            let mut call_start = (from + pos).saturating_sub(1);
            while call_start > 0 {
                let ch = s.as_bytes()[call_start - 1] as char;
                if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                    call_start -= 1;
                } else {
                    break;
                }
            }
            let cand = (idx + 1, *cat, call_start);
            best = match best {
                None => Some(cand),
                Some(prev) => {
                    if cand.0 < prev.0 {
                        Some(cand)
                    } else {
                        Some(prev)
                    }
                }
            };
        }
    }
    best.map(|(args_start, cat, call_pos)| (cat, args_start, call_pos))
}

#[inline]
fn next_char_boundary(s: &str, mut i: usize) -> usize {
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

fn extract_first_csharp_string(s: &str, mut i: usize) -> Option<(String, usize)> {
    // Skip whitespace
    let b = s.as_bytes();
    while i < b.len() && b[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= b.len() {
        return None;
    }
    // Handle regular, verbatim (@".."), or raw (""" .. """) strings
    if i + 1 < b.len() && b[i] == b'@' && b[i + 1] == b'"' {
        // verbatim string: @" ... "" ... "
        i += 2; // skip @"
        let start = i;
        let mut j = i;
        while j < b.len() {
            if b[j] == b'"' {
                // doubled quotes inside string
                if j + 1 < b.len() && b[j + 1] == b'"' {
                    j += 2;
                    continue;
                }
                let bytes = &b[start..j];
                let literal = String::from_utf8_lossy(bytes).to_string();
                return Some((literal, j + 1));
            }
            j += 1;
        }
        None
    } else if b[i] == b'"' {
        // Could be raw or regular
        // Count consecutive quotes
        let mut q = 0usize;
        while i + q < b.len() && b[i + q] == b'"' {
            q += 1;
        }
        if q >= 3 {
            // raw string with q quotes as delimiter
            let start = i + q;
            // Search for q consecutive '"'
            let mut j = start;
            'outer: while j < b.len() {
                if b[j] == b'"' {
                    // count run
                    let mut run = 1usize;
                    while j + run < b.len() && b[j + run] == b'"' {
                        run += 1;
                    }
                    if run >= q {
                        let end = j;
                        let bytes = &b[start..end];
                        let literal = String::from_utf8_lossy(bytes).to_string();
                        return Some((literal, j + q));
                    }
                    j += run;
                    continue 'outer;
                }
                j += 1;
            }
            None
        } else {
            // regular string with escapes
            i += 1; // skip opening quote
            let start = i;
            let mut j = i;
            let mut escaped = false;
            while j < b.len() {
                let c = b[j];
                if escaped {
                    escaped = false;
                    j += 1;
                    continue;
                }
                if c == b'\\' {
                    escaped = true;
                    j += 1;
                    continue;
                }
                if c == b'"' {
                    let raw_bytes = &b[start..j];
                    let raw_str = String::from_utf8_lossy(raw_bytes).to_string();
                    let unesc = unescape_csharp_string(&raw_str);
                    return Some((unesc, j + 1));
                }
                j += 1;
            }
            None
        }
    } else {
        None
    }
}

fn unescape_csharp_string(raw: &str) -> String {
    // Minimal unescape for common escapes; not exhaustive.
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('0') => out.push('\0'),
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some('u') => {
                // \uXXXX
                let mut hex = String::new();
                for _ in 0..4 {
                    if let Some(h) = chars.next() {
                        hex.push(h);
                    }
                }
                if let Ok(v) = u32::from_str_radix(&hex, 16) {
                    if let Some(ch) = char::from_u32(v) {
                        out.push(ch);
                    }
                }
            }
            Some(other) => {
                out.push(other);
            }
            None => {}
        }
    }
    out
}

fn has_following_nonws_comma_before_paren(s: &str, mut i: usize) -> bool {
    // i is index right after the closing quote; scan until ')' and see if a comma occurs before first ')'
    let b = s.as_bytes();
    while i < b.len() && b[i].is_ascii_whitespace() {
        i += 1;
    }
    if i < b.len() && b[i] == b',' {
        return true;
    }
    // Skip over possible trivia until )
    while i < b.len() {
        let c = b[i] as char;
        if c == ')' {
            return false;
        }
        if c == ',' {
            return true;
        }
        i += 1;
    }
    false
}

pub fn sanitize_mod_name(stem: &str) -> String {
    let s = utility::to_snake_case(stem);
    if s.is_empty() { "tests".to_string() } else { s }
}

pub fn write_group(
    dst_dir: &Path,
    module_name: &str,
    source_stem: &str,
    tests: &[ExtractedTest],
) -> anyhow::Result<()> {
    use std::io::Write;
    let path = dst_dir.join(format!("{}.rs", module_name));
    let mut f = std::fs::File::create(&path)?;
    // Determine which imports are needed based on test categories
    let has_stmt = tests
        .iter()
        .any(|t| matches!(t.category, Category::Statement));
    let has_non_stmt = tests.iter().any(|t| {
        matches!(
            t.category,
            Category::Tree | Category::Declaration | Category::Expression
        )
    });

    writeln!(f, "// Auto-generated from Roslyn: {}", source_stem)?;
    // Common imports
    writeln!(f, "use bsharp_parser::syntax::span::Span;")?;
    writeln!(f, "use crate::custom_asserts::after_parse;")?;
    writeln!(f, "use crate::custom_asserts::after_parse::CaseData;")?;
    // Conditional imports to avoid unused-import warnings
    if has_non_stmt {
        writeln!(f, "use bsharp_parser::bsharp::parse_csharp_source_strict;")?;
    }
    if has_stmt {
        writeln!(
            f,
            "use bsharp_parser::statement_parser::parse_statement_ws;"
        )?;
    }
    writeln!(
        f,
        "use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;"
    )?;

    // Prepare per-method occurrence counting to avoid duplicate names
    use std::collections::HashMap;
    // no local fs imports; use fully-qualified std::fs::File above
    let mut occurrences: HashMap<String, usize> = HashMap::new();
    for (idx, t) in tests.iter().enumerate() {
        let idx1 = idx + 1;
        let base = if let Some(m) = &t.method_name {
            let stripped = utility::strip_leading_test(m);
            let snake = utility::to_snake_case(stripped);
            if snake.is_empty() {
                format!("case_{}", idx1)
            } else {
                snake
            }
        } else {
            format!("case_{}", idx1)
        };
        let entry = occurrences.entry(base.clone()).or_insert(0);
        *entry += 1;
        let fn_name = if *entry == 1 {
            base.clone()
        } else {
            format!("{}_case_{}", base, *entry)
        };
        let roslyn_method = t.method_name.as_deref().unwrap_or("");
        writeln!(
            f,
            "/// Roslyn: {}.{} (case {})",
            source_stem, roslyn_method, idx1
        )?;
        writeln!(f, "#[test]")?;
        writeln!(f, "fn {}() {{", fn_name)?;
        // Use raw string literal for Rust to preserve content
        let rust_literal = to_rust_raw_string(&t.code);
        writeln!(f, "    let src = {};", rust_literal)?;
        // Expected diagnostics (count-only for now)
        if let Some(n) = t.expected_diag_count {
            writeln!(
                f,
                "    let expected: Option<ExpectedDiagnostics> = Some(ExpectedDiagnostics {{ count: {}, items: vec![] }});",
                n
            )?;
        } else {
            writeln!(f, "    let expected: Option<ExpectedDiagnostics> = None;")?;
        }
        writeln!(f, "    let span = Span::new(src);")?;
        match t.category {
            Category::Tree | Category::Declaration | Category::Expression => {
                // For Declaration/Expression we wrap as needed
                if matches!(t.category, Category::Tree) {
                    writeln!(f, "    let r = parse_csharp_source_strict(span);")?;
                } else if matches!(t.category, Category::Declaration) {
                    let wrapped = wrap_declaration(&t.code);
                    let lit = to_rust_raw_string(&wrapped);
                    writeln!(f, "    let src2 = {};", lit)?;
                    writeln!(f, "    let span2 = Span::new(src2);")?;
                    writeln!(f, "    let r = parse_csharp_source_strict(span2);")?;
                } else {
                    // Expression
                    let wrapped = wrap_expression(&t.code);
                    let lit = to_rust_raw_string(&wrapped);
                    writeln!(f, "    let src2 = {};", lit)?;
                    writeln!(f, "    let span2 = Span::new(src2);")?;
                    writeln!(f, "    let r = parse_csharp_source_strict(span2);")?;
                }
                writeln!(f, "    if let Some(expected) = expected {{")?;
                writeln!(f, "        match r {{")?;
                writeln!(f, "            Ok((_rest, unit)) => {{")?;
                if matches!(t.category, Category::Tree) {
                    writeln!(
                        f,
                        "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, expected.clone(), CaseData::File {{ unit: &unit, src, original: None }});",
                        module_name, source_stem, roslyn_method, idx1
                    )?;
                } else {
                    writeln!(
                        f,
                        "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, expected.clone(), CaseData::File {{ unit: &unit, src: src2, original: Some(src) }});",
                        module_name, source_stem, roslyn_method, idx1
                    )?;
                }
                writeln!(f, "            }}")?;
                writeln!(f, "            Err(_) => {{")?;
                writeln!(
                    f,
                    "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, expected.clone(), CaseData::Empty);",
                    module_name, source_stem, roslyn_method, idx1
                )?;
                writeln!(f, "            }}")?;
                writeln!(f, "        }}")?;
                writeln!(f, "    }} else {{")?;
                writeln!(
                    f,
                    "        assert!(r.is_ok(), \"parse failed: {{:?}}\", r.err());"
                )?;
                writeln!(f, "        let (_rest, unit) = r.unwrap();")?;
                if matches!(t.category, Category::Tree) {
                    writeln!(
                        f,
                        "        after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, None, CaseData::File {{ unit: &unit, src, original: None }});",
                        module_name, source_stem, roslyn_method, idx1
                    )?;
                } else {
                    writeln!(
                        f,
                        "        after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, None, CaseData::File {{ unit: &unit, src: src2, original: Some(src) }});",
                        module_name, source_stem, roslyn_method, idx1
                    )?;
                }
                writeln!(f, "    }}")?;
            }
            Category::Statement => {
                writeln!(f, "    let r = parse_statement_ws(span);")?;
                writeln!(f, "    if let Some(expected) = expected {{")?;
                writeln!(f, "        match r {{")?;
                writeln!(f, "            Ok((rest, ast)) => {{")?;
                writeln!(
                    f,
                    "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, expected.clone(), CaseData::Statement {{ ast: &ast, src }});",
                    module_name, source_stem, roslyn_method, idx1
                )?;
                writeln!(f, "            }}")?;
                writeln!(f, "            Err(_) => {{")?;
                writeln!(
                    f,
                    "                after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, expected.clone(), CaseData::Empty);",
                    module_name, source_stem, roslyn_method, idx1
                )?;
                writeln!(f, "            }}")?;
                writeln!(f, "        }}")?;
                writeln!(f, "    }} else {{")?;
                writeln!(
                    f,
                    "        assert!(r.is_ok(), \"parse failed: {{:?}}\", r.err());"
                )?;
                writeln!(f, "        let (rest, ast) = r.unwrap();")?;
                writeln!(
                    f,
                    "        assert!(rest.fragment().trim().is_empty(), \"Unconsumed input: {{}}\", rest.fragment());"
                )?;
                writeln!(
                    f,
                    "        after_parse::after_parse_with_expected(\"{}\", \"{}\", \"{}\", {}, None, CaseData::Statement {{ ast: &ast, src }});",
                    module_name, source_stem, roslyn_method, idx1
                )?;
                writeln!(f, "    }}")?;
            }
        }
        writeln!(f, "}}\n")?;
    }
    Ok(())
}

pub fn wrap_declaration(code: &str) -> String {
    let is_top = code.trim_start().starts_with("class ")
        || code.trim_start().starts_with("struct ")
        || code.trim_start().starts_with("interface ")
        || code.trim_start().starts_with("record ")
        || code.trim_start().starts_with("enum ")
        || code.trim_start().starts_with("namespace ");
    if is_top {
        code.to_string()
    } else {
        format!("class C {{ {} }}", code)
    }
}

pub fn wrap_expression(code: &str) -> String {
    format!("class C {{ void M() {{ {}; }} }}", code)
}

fn to_rust_raw_string(s: &str) -> String {
    // Choose number of # that avoids collisions
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
