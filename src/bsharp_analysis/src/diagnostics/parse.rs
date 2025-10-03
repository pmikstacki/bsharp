use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation, StackContext};

/// Summarize deepest location (line, column, line_text) from an ErrorTree
pub fn summarize_location<'a>(input: &'a str, err: &ErrorTree<&'a str>) -> (usize, usize, String) {
    fn byte_offset(input: &str, location: &str) -> usize {
        let ip = input.as_ptr() as usize;
        let lp = location.as_ptr() as usize;
        lp.saturating_sub(ip)
    }
    fn line_col(input: &str, offset: usize) -> (usize, usize) {
        let mut line = 1usize;
        let mut col = 1usize;
        for (i, ch) in input.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }
    fn line_slice(input: &str, line_no: usize) -> String {
        input
            .lines()
            .nth(line_no.saturating_sub(1))
            .unwrap_or("")
            .to_string()
    }
    fn deepest_base<'a>(input: &'a str, e: &'a ErrorTree<&'a str>) -> (&'a str, usize) {
        match e {
            ErrorTree::Base { location, .. } => (*location, byte_offset(input, location)),
            ErrorTree::Stack { base, .. } => deepest_base(input, base),
            ErrorTree::Alt(list) => {
                let mut best_loc: Option<(&str, usize)> = None;
                for child in list {
                    let cand = deepest_base(input, child);
                    if best_loc.map(|(_, off)| cand.1 > off).unwrap_or(true) {
                        best_loc = Some(cand);
                    }
                }
                best_loc.unwrap_or((input, 0))
            }
        }
    }
    let (_loc, off) = deepest_base(input, err);
    let (line, col) = line_col(input, off);
    (line, col, line_slice(input, line))
}

/// Summarize expected/found info for JSON payloads
pub fn summarize_expected_found(err: &ErrorTree<&str>) -> (String, String) {
    match err {
        ErrorTree::Base { kind, .. } => match kind {
            BaseErrorKind::Expected(exp) => (format!("{}", exp), String::new()),
            other => (format!("{:?}", other), String::new()),
        },
        ErrorTree::Stack { base, .. } => summarize_expected_found(base),
        ErrorTree::Alt(list) => {
            if let Some(first) = list.first() {
                summarize_expected_found(first)
            } else {
                (String::new(), String::new())
            }
        }
    }
}

/// Render a concise pretty string for the deepest parse error, including caret.
pub fn render_pretty_parse_error(input: &str, err: &ErrorTree<&str>) -> String {
    fn byte_offset(input: &str, location: &str) -> usize {
        let ip = input.as_ptr() as usize;
        let lp = location.as_ptr() as usize;
        lp.saturating_sub(ip)
    }
    fn line_col(input: &str, offset: usize) -> (usize, usize) {
        let mut line = 1usize;
        let mut col = 1usize;
        for (i, ch) in input.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }
    fn line_slice(input: &str, line_no: usize) -> String {
        input
            .lines()
            .nth(line_no.saturating_sub(1))
            .unwrap_or("")
            .to_string()
    }
    #[derive(Clone)]
    struct BaseInfo {
        expected: Option<String>,
        offset: usize,
        contexts: Vec<String>,
    }
    fn collect<'a>(
        input: &'a str,
        e: &'a ErrorTree<&'a str>,
        ctx: &mut Vec<String>,
        out: &mut Vec<BaseInfo>,
    ) {
        match e {
            ErrorTree::Base { location, kind } => {
                let expected = match kind {
                    BaseErrorKind::Expected(exp) => match exp {
                        Expectation::Tag(s) => Some((*s).to_string()),
                        Expectation::Char(c) => Some(format!("'{}'", c)),
                        Expectation::Eof => Some("<eof>".to_string()),
                        other => Some(format!("{}", other)),
                    },
                    other => Some(format!("{:?}", other)),
                };
                out.push(BaseInfo {
                    expected,
                    offset: byte_offset(input, location),
                    contexts: ctx.clone(),
                });
            }
            ErrorTree::Stack { base, contexts } => {
                let base_ctx_len = ctx.len();
                for (_loc, sc) in contexts {
                    match sc {
                        StackContext::Context(name) => ctx.push((*name).to_string()),
                        StackContext::Kind(kind) => ctx.push(format!("{:?}", kind)),
                    }
                }
                collect(input, base, ctx, out);
                ctx.truncate(base_ctx_len);
            }
            ErrorTree::Alt(list) => {
                for child in list {
                    collect(input, child, ctx, out);
                }
            }
        }
    }
    let mut bases: Vec<BaseInfo> = Vec::new();
    collect(input, err, &mut Vec::new(), &mut bases);
    let is_toplevel = |expected: &Option<String>| -> bool {
        if let Some(s) = expected.as_ref() {
            matches!(
                s.as_str(),
                "class" | "struct" | "interface" | "record" | "enum" | "namespace" | "<eof>"
            )
        } else {
            false
        }
    };
    let is_token = |expected: &Option<String>, ch: char| -> bool {
        if let Some(s) = expected.as_ref() {
            if s.len() == 1 && s.starts_with(ch) {
                return true;
            }
            if s.len() == 3
                && s.starts_with('\'')
                && s.ends_with('\'')
                && s.chars().nth(1) == Some(ch)
            {
                return true;
            }
        }
        false
    };
    bases.sort_by_key(|info| info.offset);

    // Early generic heuristic: if the deepest error is on a line that looks like `... = ... }`,
    // prefer reporting a missing semicolon just before the closing brace.
    if let Some(max_info) = bases.iter().max_by_key(|info| info.offset) {
        let (line, _col) = line_col(input, max_info.offset);
        let line_text = line_slice(input, line);
        if line_text.contains('}')
            && line_text.contains('=')
            && !line_text.contains("if (")
            && !line_text.contains("else")
        {
            // Caret: just before the first closing brace on this line
            if let Some(brace_idx) = line_text.find('}') {
                let caret_col = line_text[..brace_idx].chars().count() + 1;
                return format!(
                    "at {}:{}: expected ';'\n{}\n{}^",
                    line,
                    caret_col,
                    line_text,
                    " ".repeat(caret_col.saturating_sub(1))
                );
            }
        }
    }

    let has_after_paren = |info: &BaseInfo| info.contexts.iter().any(|c| c == "after_paren");
    let has_after_else = |info: &BaseInfo| info.contexts.iter().any(|c| c == "after_else");

    if let Some(info) = bases
        .iter()
        .filter(|info| has_after_paren(info) || has_after_else(info))
        .min_by_key(|info| info.offset)
    {
        let (line, col) = line_col(input, info.offset);
        let line_text = line_slice(input, line);
        return format!(
            "at {}:{}: expected {}\n{}\n{}^",
            line,
            col,
            "statement",
            line_text,
            " ".repeat(col.saturating_sub(1))
        );
    }

    // Prefer earliest 'statement' expectation (missing body) unconditionally
    if let Some(info) = bases
        .iter()
        .filter(|info| info.expected.as_deref() == Some("statement"))
        .min_by_key(|info| info.offset)
    {
        let (line, col) = line_col(input, info.offset);
        let line_text = line_slice(input, line);
        return format!(
            "at {}:{}: expected {}\n{}\n{}^",
            line,
            col,
            "statement",
            line_text,
            " ".repeat(col.saturating_sub(1))
        );
    }

    // Early rule for common pattern: missing semicolon before '}' on the same line (e.g., "int x = 1 }")
    if let Some(info) = bases
        .iter()
        .filter(|info| is_token(&info.expected, '}'))
        .min_by_key(|info| info.offset)
    {
        let off = info.offset;
        let (line, _col) = line_col(input, off);
        let line_text = line_slice(input, line);
        if line_text.contains('=') && !line_text.contains("if (") && !line_text.contains("else") {
            // caret after last non-whitespace before the brace
            let bytes = input.as_bytes();
            let mut k = off.saturating_sub(1);
            while k > 0 && matches!(bytes[k], b' ' | b'\t' | b'\r' | b'\n') {
                k = k.saturating_sub(1);
            }
            let target_off = k.saturating_add(1);
            let (line2, col2) = line_col(input, target_off);
            let line_text2 = line_slice(input, line2);
            return format!(
                "at {}:{}: expected ';'\n{}\n{}^",
                line2,
                col2,
                line_text2,
                " ".repeat(col2.saturating_sub(1))
            );
        }
    }

    // Prefer explicit semicolon expectation if present
    if let Some(first_sc) = bases
        .iter()
        .filter(|info| is_token(&info.expected, ';'))
        .min_by_key(|info| info.offset)
    {
        let off = first_sc.offset;
        let (line, col) = line_col(input, off);
        let line_text = line_slice(input, line);
        // Prefer 'statement' wording on lines that clearly show if/else headers
        if line_text.contains("if (") || line_text.contains("else") {
            return format!(
                "at {}:{}: expected {}\n{}\n{}^",
                line,
                col,
                "statement",
                line_text,
                " ".repeat(col.saturating_sub(1))
            );
        }
        return format!(
            "at {}:{}: expected ';'\n{}\n{}^",
            line,
            col,
            line_text,
            " ".repeat(col.saturating_sub(1))
        );
    }

    // If there's an expected '}' near the end of a control header, prefer 'expected statement';
    // otherwise synthesize ';' when the previous token looks like an expression end; fall back to reporting '}'.
    if let Some(info) = bases
        .iter()
        .filter(|info| is_token(&info.expected, '}'))
        .max_by_key(|info| info.offset)
        .cloned()
    {
        let bytes = input.as_bytes();
        // Find last non-whitespace character before off
        let mut j = info.offset.saturating_sub(1);
        while j > 0 && matches!(bytes[j], b' ' | b'\t' | b'\r' | b'\n') {
            j = j.saturating_sub(1);
        }
        let last_non_ws = bytes.get(j).copied();
        let after_paren = last_non_ws == Some(b')');
        let after_else = if !after_paren {
            // Try to read the preceding word (like 'else') if any
            let mut end = j;
            while end > 0 && bytes[end].is_ascii_alphabetic() {
                end = end.saturating_sub(1);
            }
            let start = if bytes
                .get(end)
                .map(|c| c.is_ascii_alphabetic())
                .unwrap_or(false)
            {
                end
            } else {
                end.saturating_add(1)
            };
            let slice = if start <= j { &input[start..=j] } else { "" };
            slice == "else"
        } else {
            false
        };
        if after_paren || after_else {
            let (line, col) = line_col(input, info.offset);
            let line_text = line_slice(input, line);
            return format!(
                "at {}:{}: expected {}\n{}\n{}^",
                line,
                col,
                "statement",
                line_text,
                " ".repeat(col.saturating_sub(1))
            );
        } else {
            // Decide between synthesizing ';' vs reporting '}'
            let prev_ch = last_non_ws;
            let looks_like_expr_end = matches!(
                prev_ch,
                Some(c)
                    if c.is_ascii_alphanumeric()
                        || c == b')'
                        || c == b']'
                        || c == b'"'
                        || c == b'\''
            );
            if looks_like_expr_end {
                let mut k = info.offset.saturating_sub(1);
                while k > 0 && matches!(bytes[k], b' ' | b'\t' | b'\r' | b'\n') {
                    k = k.saturating_sub(1);
                }
                let target_off = k.saturating_add(1);
                let (line, col) = line_col(input, target_off);
                let line_text = line_slice(input, line);
                if line_text.contains("if (") || line_text.contains("else") {
                    return format!(
                        "at {}:{}: expected {}\n{}\n{}^",
                        line,
                        col,
                        "statement",
                        line_text,
                        " ".repeat(col.saturating_sub(1))
                    );
                }
                // Prefer ';' when the line has an assignment, which strongly suggests an expression statement
                if line_text.contains("=")
                    || line_text.contains("let")
                    || line_text.contains("const")
                {
                    return format!(
                        "at {}:{}: expected ';'\n{}\n{}^",
                        line,
                        col,
                        line_text,
                        " ".repeat(col.saturating_sub(1))
                    );
                }
                return format!(
                    "at {}:{}: expected ';'\n{}\n{}^",
                    line,
                    col,
                    line_text,
                    " ".repeat(col.saturating_sub(1))
                );
            } else {
                let (line, col) = line_col(input, info.offset);
                let line_text = line_slice(input, line);
                if line_text.contains("if (") || line_text.contains("else") {
                    return format!(
                        "at {}:{}: expected {}\n{}\n{}^",
                        line,
                        col,
                        "statement",
                        line_text,
                        " ".repeat(col.saturating_sub(1))
                    );
                }
                // If the part of the line before the brace clearly contains an assignment, prefer ';'
                if line_text.contains("=") {
                    return format!(
                        "at {}:{}: expected ';'\n{}\n{}^",
                        line,
                        col,
                        line_text,
                        " ".repeat(col.saturating_sub(1))
                    );
                }
                return format!(
                    "at {}:{}: expected '}}'\n{}\n{}^",
                    line,
                    col,
                    line_text,
                    " ".repeat(col.saturating_sub(1))
                );
            }
        }
    }

    // Fallback: deepest non top-level
    let chosen = bases
        .iter()
        .rev()
        .find(|info| !is_toplevel(&info.expected))
        .or_else(|| bases.last());
    let info = chosen.cloned().unwrap_or(BaseInfo {
        expected: Some("<eof>".to_string()),
        offset: 0,
        contexts: Vec::new(),
    });
    let (line, col) = line_col(input, info.offset);
    let line_text = line_slice(input, line);
    let expected_text = info.expected.unwrap_or_else(|| "token".to_string());
    format!(
        "at {}:{}: expected {}\n{}\n{}^",
        line,
        col,
        expected_text,
        line_text,
        " ".repeat(col.saturating_sub(1))
    )
}
