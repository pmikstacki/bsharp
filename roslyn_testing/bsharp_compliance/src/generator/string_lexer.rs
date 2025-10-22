pub fn extract_first_string(s: &str, mut i: usize) -> Option<(String, usize)> {
    let b = s.as_bytes();
    while i < b.len() && b[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= b.len() {
        return None;
    }
    if i + 1 < b.len() && b[i] == b'@' && b[i + 1] == b'"' {
        // verbatim string
        i += 2;
        let start = i;
        let mut j = i;
        while j < b.len() {
            if b[j] == b'"' {
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
        // raw or regular strings
        let mut q = 0usize;
        while i + q < b.len() && b[i + q] == b'"' {
            q += 1;
        }
        if q >= 3 {
            let start = i + q;
            let mut j = start;
            'outer: while j < b.len() {
                if b[j] == b'"' {
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
            i += 1;
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
                    let raw = String::from_utf8_lossy(&b[start..j]).to_string();
                    return Some((unescape_regular_string(&raw), j + 1));
                }
                j += 1;
            }
            None
        }
    } else {
        None
    }
}

fn unescape_regular_string(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut it = raw.chars().peekable();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('0') => out.push('\0'),
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some('u') => {
                let mut hex = String::new();
                for _ in 0..4 {
                    if let Some(h) = it.next() {
                        hex.push(h);
                    }
                }
                if let Ok(v) = u32::from_str_radix(&hex, 16) {
                    if let Some(ch) = char::from_u32(v) {
                        out.push(ch);
                    }
                }
            }
            Some(other) => out.push(other),
            None => {}
        }
    }
    out
}

pub fn find_call_closing_paren(s: &str, open_paren_idx: usize) -> Option<usize> {
    let b = s.as_bytes();
    if open_paren_idx >= b.len() || b[open_paren_idx] != b'(' {
        return None;
    }
    let mut i = open_paren_idx;
    let mut depth: i32 = 0;
    while i < b.len() {
        let c = b[i] as char;
        if c == '@' && i + 1 < b.len() && b[i + 1] == b'"' {
            if let Some(j) = skip_csharp_verbatim_string(s, i) {
                i = j;
                continue;
            }
        }
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

pub fn has_following_nonws_comma_before_paren(s: &str, mut i: usize) -> bool {
    let b = s.as_bytes();
    while i < b.len() && b[i].is_ascii_whitespace() {
        i += 1;
    }
    if i < b.len() && b[i] == b',' {
        return true;
    }
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
