// Scanner: find Using*/Parse* invocations and return positions.

#[derive(Debug, Clone)]
pub struct CallHit {
    pub start_args: usize,
    pub call_pos: usize,
    pub kind: CallKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallKind {
    UsingTree,
    UsingStatement,
    UsingDeclaration,
    UsingExpression,
    ParseCompilationUnit,
    ParseTree,
    ParseStatement,
    ParseExpression,
    ParseMemberDeclaration,
    ParseName,
    ParseTypeName,
    ParseParameterList,
    ParseAttributeList,
}

pub fn find_next_call(content: &str, from: usize) -> Option<CallHit> {
    let from = next_char_boundary(content, from);
    let hay = &content[from..];

    // Using* patterns (unqualified identifiers)
    let using_patterns: &[(&str, CallKind)] = &[
        ("UsingTree", CallKind::UsingTree),
        ("UsingStatement", CallKind::UsingStatement),
        ("UsingDeclaration", CallKind::UsingDeclaration),
        ("UsingExpression", CallKind::UsingExpression),
    ];
    let mut candidates: Vec<(usize, CallKind, usize)> = Vec::new();
    for (kw, kind) in using_patterns {
        if let Some(pos) = hay.find(kw) {
            let mut idx = from + pos + kw.len();
            // skip whitespace
            while idx < content.len() && content.as_bytes()[idx].is_ascii_whitespace() { idx += 1; }
            // optional generic args <...>
            if idx < content.len() && content.as_bytes()[idx] == b'<' {
                idx = skip_generics(content, idx);
                // whitespace after generics
                while idx < content.len() && content.as_bytes()[idx].is_ascii_whitespace() { idx += 1; }
            }
            if idx < content.len() && content.as_bytes()[idx] == b'(' {
                let start_args = idx + 1;
                let call_pos = rewind_ident_start(content, (from + pos));
                candidates.push((start_args, *kind, call_pos));
            }
        }
    }

    // Parse* patterns (possibly qualified with dots)
    let parse_patterns: &[(&str, CallKind)] = &[
        ("ParseCompilationUnit", CallKind::ParseCompilationUnit),
        ("ParseTree", CallKind::ParseTree),
        ("ParseStatement", CallKind::ParseStatement),
        ("ParseExpression", CallKind::ParseExpression),
        ("ParseMemberDeclaration", CallKind::ParseMemberDeclaration),
        ("ParseName", CallKind::ParseName),
        ("ParseTypeName", CallKind::ParseTypeName),
        ("ParseParameterList", CallKind::ParseParameterList),
        ("ParseAttributeList", CallKind::ParseAttributeList),
    ];
    for (kw, kind) in parse_patterns {
        if let Some(pos) = hay.find(kw) {
            let mut idx = from + pos + kw.len();
            while idx < content.len() && content.as_bytes()[idx].is_ascii_whitespace() { idx += 1; }
            if idx < content.len() && content.as_bytes()[idx] == b'<' {
                idx = skip_generics(content, idx);
                while idx < content.len() && content.as_bytes()[idx].is_ascii_whitespace() { idx += 1; }
            }
            if idx < content.len() && content.as_bytes()[idx] == b'(' {
                let start_args = idx + 1;
                let call_pos = rewind_qualified_start(content, (from + pos));
                candidates.push((start_args, *kind, call_pos));
            }
        }
    }

    if candidates.is_empty() { return None; }
    candidates.sort_by_key(|(start_args, _, _)| *start_args);
    let (start_args, kind, call_pos) = candidates[0];
    Some(CallHit { start_args, call_pos, kind })
}

fn next_char_boundary(s: &str, mut i: usize) -> usize {
    while i < s.len() && !s.is_char_boundary(i) { i += 1; }
    i
}

fn skip_generics(s: &str, mut i: usize) -> usize {
    // i points at '<'
    let b = s.as_bytes();
    let mut depth = 0i32;
    while i < b.len() {
        let ch = b[i] as char;
        if ch == '<' { depth += 1; }
        else if ch == '>' { depth -= 1; if depth == 0 { i += 1; break; } }
        i += 1;
    }
    i
}

fn rewind_ident_start(s: &str, mut i: usize) -> usize {
    // Rewind to start of identifier
    while i > 0 {
        let ch = s.as_bytes()[i.saturating_sub(1)] as char;
        if ch.is_alphanumeric() || ch == '_' { i -= 1; } else { break; }
    }
    i
}

fn rewind_qualified_start(s: &str, mut i: usize) -> usize {
    // Rewind across dotted qualifiers
    while i > 0 {
        let ch = s.as_bytes()[i.saturating_sub(1)] as char;
        if ch.is_alphanumeric() || ch == '_' || ch == '.' { i -= 1; } else { break; }
    }
    i
}
