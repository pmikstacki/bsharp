// Parse trailing Roslyn helper arguments. Start with counting Diagnostic(...) occurrences.

#[derive(Debug, Clone, Default)]
pub struct DiagnosticsParseResult {
    pub count: usize,
}

pub fn count_diagnostic_invocations(s: &str) -> usize {
    let needle = b"Diagnostic(";
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut count = 0usize;
    while i + needle.len() <= bytes.len() {
        if &bytes[i..i + needle.len()] == needle {
            let prev = if i == 0 { None } else { Some(bytes[i - 1]) };
            if is_ident_boundary(prev) { count += 1; }
            i += needle.len();
            continue;
        }
        i += 1;
    }
    count
}

fn is_ident_boundary(ch: Option<u8>) -> bool {
    match ch {
        Some(c) => !(c as char).is_alphanumeric() && c != b'_',
        None => true,
    }
}

// --- Structured extraction (best-effort) ---
use crate::generator::model::RoslynDiagnosticExpectation;
use crate::generator::string_lexer::find_call_closing_paren;

pub fn extract_expected_diagnostics(args: &str) -> Vec<RoslynDiagnosticExpectation> {
    let mut out = Vec::new();
    let bytes = args.as_bytes();
    let mut i = 0usize;
    while i + 11 <= bytes.len() { // len("Diagnostic(") == 11
        if bytes[i..].starts_with(b"Diagnostic(") && is_ident_boundary(i.checked_sub(1).map(|j| bytes[j])) {
            // find matching ) for this Diagnostic(
            let open = i + "Diagnostic".len();
            let Some(open_paren) = next_non_ws(args, open) else { break; };
            let Some(close_paren) = find_call_closing_paren(args, open_paren) else { i += 1; continue; };
            let inner = &args[(open_paren + 1)..close_paren];

            let mut item = RoslynDiagnosticExpectation::default();
            item.code = parse_error_code(inner);
            item.message_args = parse_message_args(inner);

            // parse chained methods after Diagnostic(...)
            let mut j = close_paren + 1;
            while let Some((name, args_start, args_end, next_idx)) = try_parse_chain(args, j) {
                match name {
                    "WithLocation" | "WithSpan" | "WithPosition" => {
                        if let Some((a, b)) = parse_two_usize(&args[args_start..args_end]) { item.span = Some((a, b)); }
                    }
                    _ => {}
                }
                j = next_idx;
            }

            out.push(item);
            i = close_paren + 1;
            continue;
        }
        i += 1;
    }
    out
}

fn next_non_ws(s: &str, i: usize) -> Option<usize> {
    let b = s.as_bytes();
    let mut j = i;
    while j < b.len() && b[j].is_ascii_whitespace() { j += 1; }
    if j < b.len() && b[j] == b'(' { Some(j) } else { None }
}

fn parse_error_code(inner: &str) -> Option<String> {
    // First arg until top-level comma or end
    let mut depth = 0i32;
    let mut j = 0usize;
    let bytes = inner.as_bytes();
    while j < bytes.len() {
        let c = bytes[j] as char;
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => break,
            '"' => { j = skip_string(inner, j); continue; }
            '@' => { if j + 1 < bytes.len() && bytes[j+1] == b'"' { j = skip_verbatim_string(inner, j); continue; } }
            _ => {}
        }
        j += 1;
    }
    let token = inner[..j].trim();
    if token.is_empty() { return None; }
    // Accept ErrorCode.ERR_Xxx or ERR_Xxx
    let code = token.rsplit('.').next().unwrap_or(token).trim();
    if code.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') { Some(code.to_string()) } else { None }
}

fn parse_message_args(inner: &str) -> Vec<String> {
    // Simple heuristic: arguments after first ',' at top-level
    let mut depth = 0i32;
    let mut j = 0usize;
    let bytes = inner.as_bytes();
    while j < bytes.len() {
        let c = bytes[j] as char;
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => { j += 1; break; }
            '"' => { j = skip_string(inner, j); continue; }
            '@' => { if j + 1 < bytes.len() && bytes[j+1] == b'"' { j = skip_verbatim_string(inner, j); continue; } }
            _ => {}
        }
        j += 1;
    }
    if j >= bytes.len() { return Vec::new(); }
    let rest = inner[j..].trim();
    if rest.is_empty() { return Vec::new(); }
    vec![rest.to_string()]
}

fn try_parse_chain(s: &str, mut i: usize) -> Option<(&str, usize, usize, usize)> {
    let b = s.as_bytes();
    // skip whitespace
    while i < b.len() && b[i].is_ascii_whitespace() { i += 1; }
    if i >= b.len() || b[i] != b'.' { return None; }
    i += 1;
    // read ident
    let start_ident = i;
    while i < b.len() && ((b[i] as char).is_ascii_alphanumeric() || b[i] == b'_') { i += 1; }
    if i >= b.len() || start_ident == i { return None; }
    let name = &s[start_ident..i];
    while i < b.len() && b[i].is_ascii_whitespace() { i += 1; }
    if i >= b.len() || b[i] != b'(' { return None; }
    let open_paren = i;
    let Some(close_paren) = find_call_closing_paren(s, open_paren) else { return None; };
    let args_start = open_paren + 1;
    let args_end = close_paren;
    let next_idx = close_paren + 1;
    Some((name, args_start, args_end, next_idx))
}

fn parse_two_usize(s: &str) -> Option<(usize, usize)> {
    let mut parts = s.split(',').map(|t| t.trim());
    let a = parts.next()?.parse::<usize>().ok()?;
    let b = parts.next()?.parse::<usize>().ok()?;
    Some((a, b))
}

fn skip_string(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    let mut j = i + 1;
    let mut escaped = false;
    while j < b.len() {
        let c = b[j];
        if escaped { escaped = false; j += 1; continue; }
        if c == b'\\' { escaped = true; j += 1; continue; }
        if c == b'"' { return j + 1; }
        j += 1;
    }
    j
}

fn skip_verbatim_string(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    let mut j = i + 2; // after @"
    while j < b.len() {
        if b[j] == b'"' {
            if j + 1 < b.len() && b[j + 1] == b'"' { j += 2; continue; }
            return j + 1;
        }
        j += 1;
    }
    j
}
