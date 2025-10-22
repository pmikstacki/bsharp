#[derive(Debug, Clone)]
pub struct ExpectedNode {
    pub kind: String,
    pub token_value: Option<String>,
    pub children: Vec<ExpectedNode>,
}

#[derive(Debug, Clone)]
pub struct ExpectedTree {
    pub root: ExpectedNode,
}

#[derive(Debug, Clone)]
pub struct ExtractedStructureTest {
    pub method_name: Option<String>,
    pub src_code: String,
    pub expected: ExpectedTree,
    pub call_pos: usize,
}

pub fn extract_structure_tests(content: &str) -> Vec<ExtractedStructureTest> {
    use crate::tests_writer::utility::{collect_test_methods, find_enclosing_method_name};
    let methods = collect_test_methods(content);
    let verbose = std::env::var("BSHARP_LOG").is_ok();

    let mut out = Vec::new();
    let mut total_hits = 0usize;
    for (idx, _) in content.match_indices("UsingTree(") {
        total_hits += 1;
        if verbose {
            eprintln!("[structure][dsl] UsingTree hit at byte {}", idx);
        }
        // Extract source snippet: var text = "..."; immediately preceding the UsingTree call
        let prefix = &content[..idx];
        let mut src_code = String::new();
        if let Some(text_pos) = prefix.rfind("var text =") {
            let rest = &content[text_pos..];
            if let Some(qidx) = rest.find('"') {
                let rest2 = &rest[qidx + 1..];
                if let Some(end_rel) = rest2.find("\";") {
                    let src = &rest2[..end_rel];
                    src_code = src.to_string();
                }
            }
        }

        // Parse the DSL block starting at the UsingTree(...) call site
        let dsl_segment = &content[idx..];
        let parsed = parse_expected_tree_from_dsl(dsl_segment);
        if let Some(mut expected) = parsed {
            // Translate from Roslyn kinds to our kinds
            expected = crate::generator::kind_map::translate_expected_tree(&expected);

            // If source code is empty, normalize expected to an empty compilation unit
            if src_code.trim().is_empty() && expected.root.kind == "CompilationUnit" {
                expected.root.children.clear();
            }

            // Enrich with method name for nicer test names
            let method_name = find_enclosing_method_name(&methods, idx);

            if verbose {
                eprintln!(
                    "[structure][dsl] parsed expected tree: root={} children={}",
                    expected.root.kind,
                    expected.root.children.len()
                );
            }
            out.push(ExtractedStructureTest {
                method_name,
                src_code,
                expected,
                call_pos: idx,
            });
        } else if verbose {
            // Log a small excerpt to help debugging
            let excerpt = &dsl_segment[..dsl_segment
                .char_indices()
                .nth(1200)
                .map(|(i, _)| i)
                .unwrap_or_else(|| dsl_segment.len())];
            eprintln!(
                "[structure][dsl] parse_expected_tree_from_dsl returned None. Excerpt start:\n{}\n---",
                excerpt
            );
        }
    }
    if verbose {
        eprintln!(
            "[structure][dsl] UsingTree total hits: {} -> extracted: {}",
            total_hits,
            out.len()
        );
    }
    out
}

/// Parse the Roslyn structure DSL that follows a UsingTree(...) call.
/// Supported constructs:
/// - N(SyntaxKind.KindName[, "token"]) with optional immediate child block delimited by {...}
/// - EOF()
/// - Any amount of whitespace and newlines; 'M(...)' entries are ignored
fn parse_expected_tree_from_dsl(input: &str) -> Option<ExpectedTree> {
    let mut p = DslParser::new(input);
    p.parse()
}

struct DslParser<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> DslParser<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, pos: 0 }
    }

    #[inline]
    fn advance_one_char(&mut self) {
        if self.pos < self.s.len() {
            if let Some(ch) = self.s[self.pos..].chars().next() {
                self.pos += ch.len_utf8();
            } else {
                self.pos = self.s.len();
            }
        }
    }

    fn parse(&mut self) -> Option<ExpectedTree> {
        // Fast-forward to the first N( occurrence following the UsingTree call
        if let Some(start) = self.find_token("N(") {
            self.pos = start;
        } else {
            return None;
        }

        let verbose = std::env::var("BSHARP_LOG").is_ok();
        if verbose {
            eprintln!("[structure][dsl] parse begin at offset {}", self.pos);
        }
        let t0 = std::time::Instant::now();
        let mut last_log_pos = self.pos;

        let mut parents: Vec<ExpectedNode> = Vec::new();
        let mut root: Option<ExpectedNode> = None;

        while self.pos < self.s.len() {
            self.skip_ws();
            if verbose {
                // Log every ~50k chars progressed to show forward movement
                if self.pos.saturating_sub(last_log_pos) > 50_000 {
                    let end = (self.pos + 40).min(self.s.len());
                    let excerpt = &self.s[self.pos..end];
                    eprintln!(
                        "[structure][dsl] pos={} next='{}'",
                        self.pos,
                        excerpt.replace('\n', "\\n")
                    );
                    last_log_pos = self.pos;
                }
            }

            if self.peek_str("EOF()") {
                break;
            }

            if self.peek_str("M(") {
                // Skip missing entries entirely, including their argument list and optional semicolon
                // Advance past 'M' so skip_paren_group() can consume the '('
                self.pos += 1; // now at '('
                self.skip_paren_group();
                self.skip_ws();
                self.eat_char(';');
                continue;
            }

            if self.peek_str("N(") {
                if let Some(mut node) = self.parse_n_call() {
                    self.skip_ws();
                    if self.eat_char('{') {
                        // Opening a child block: push node as current parent
                        parents.push(node);
                    } else {
                        // Leaf node: attach to current parent or root
                        if let Some(parent) = parents.last_mut() {
                            parent.children.push(node);
                        } else if root.is_none() {
                            root = Some(node);
                        } else if let Some(r) = root.as_mut() {
                            r.children.push(node);
                        }
                    }
                    continue;
                } else {
                    break;
                }
            }

            if self.eat_char('}') {
                // Close current parent and attach upwards
                if let Some(done) = parents.pop() {
                    if let Some(parent) = parents.last_mut() {
                        parent.children.push(done);
                    } else if root.is_none() {
                        root = Some(done);
                    } else if let Some(r) = root.as_mut() {
                        r.children.push(done);
                    }
                }
                continue;
            }

            // Nothing recognized on this character, advance by one UTF-8 char
            self.advance_one_char();
        }

        self.skip_ws();
        if self.peek_str("EOF()") {
            if verbose {
                eprintln!(
                    "[structure][dsl] EOF() detected at pos {} (elapsed {:?})",
                    self.pos,
                    t0.elapsed()
                );
            }
            // Close any remaining open parents
            while let Some(done) = parents.pop() {
                if let Some(parent) = parents.last_mut() {
                    parent.children.push(done);
                } else if root.is_none() {
                    root = Some(done);
                } else if let Some(r) = root.as_mut() {
                    r.children.push(done);
                }
            }
            let out = root.map(|r| ExpectedTree { root: r });
            if verbose {
                eprintln!("[structure][dsl] parse end (elapsed {:?})", t0.elapsed());
            }
            out
        } else {
            None
        }
    }

    fn parse_n_call(&mut self) -> Option<ExpectedNode> {
        // Expect: N(SyntaxKind.KindName[, "token"]) ;
        if !self.peek_str("N(") {
            return None;
        }
        self.pos += 2; // skip N(
        self.skip_ws();
        if !self.peek_str("SyntaxKind.") {
            return None;
        }
        self.pos += "SyntaxKind.".len();
        let kind = self.read_ident();
        if kind.is_empty() {
            return None;
        }

        self.skip_ws();
        let mut token_value: Option<String> = None;
        if self.eat_char(',') {
            self.skip_ws();
            if self.eat_char('"') {
                let s = self.read_string_literal();
                token_value = Some(s);
            }
        }
        self.skip_ws();
        self.eat_char(')');
        self.skip_ws();
        self.eat_char(';');

        Some(ExpectedNode {
            kind,
            token_value,
            children: vec![],
        })
    }

    fn find_token(&self, needle: &str) -> Option<usize> {
        self.s[self.pos..].find(needle).map(|o| self.pos + o)
    }

    fn skip_ws(&mut self) {
        while self.pos < self.s.len() {
            let b = self.s.as_bytes()[self.pos];
            if b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' {
                self.pos += 1;
                continue;
            }
            // line comment
            if self.s[self.pos..].starts_with("//") {
                while self.pos < self.s.len() && self.s.as_bytes()[self.pos] != b'\n' {
                    self.pos += 1;
                }
                continue;
            }
            break;
        }
    }

    fn peek_str(&self, s2: &str) -> bool {
        if self.pos > self.s.len() {
            return false;
        }
        self.s[self.pos..].starts_with(s2)
    }

    fn eat_char(&mut self, ch: char) -> bool {
        if self.pos < self.s.len() && self.s[self.pos..].chars().next() == Some(ch) {
            self.pos += ch.len_utf8();
            true
        } else {
            false
        }
    }

    fn skip_paren_group(&mut self) {
        if !self.eat_char('(') {
            return;
        }
        let mut depth = 1i32;
        while self.pos < self.s.len() && depth > 0 {
            let c = self.s[self.pos..].chars().next().unwrap();
            self.pos += c.len_utf8();
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '"' => {
                    self.skip_string_body();
                }
                _ => {}
            }
        }
    }

    fn read_ident(&mut self) -> String {
        let mut out = String::new();
        while self.pos < self.s.len() {
            let c = self.s[self.pos..].chars().next().unwrap();
            if c.is_ascii_alphanumeric() || c == '_' {
                out.push(c);
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
        out
    }

    fn read_string_literal(&mut self) -> String {
        // starting quote already consumed
        let mut out = String::new();
        while self.pos < self.s.len() {
            let c = self.s[self.pos..].chars().next().unwrap();
            self.pos += c.len_utf8();
            match c {
                '\\' => {
                    if self.pos < self.s.len() {
                        let n = self.s[self.pos..].chars().next().unwrap();
                        self.pos += n.len_utf8();
                        out.push(match n {
                            '\\' => '\\',
                            '"' => '"',
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            other => other,
                        });
                    }
                }
                '"' => break,
                _ => out.push(c),
            }
        }
        out
    }

    fn skip_string_body(&mut self) {
        while self.pos < self.s.len() {
            let c = self.s[self.pos..].chars().next().unwrap();
            self.pos += c.len_utf8();
            match c {
                '\\' => {
                    if self.pos < self.s.len() {
                        let n = self.s[self.pos..].chars().next().unwrap();
                        self.pos += n.len_utf8();
                    }
                }
                '"' => break,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_structure_dsl() {
        let s = r#"
UsingTree(text);
N(SyntaxKind.CompilationUnit);
{
    N(SyntaxKind.ClassDeclaration);
    {
        N(SyntaxKind.ClassKeyword);
        N(SyntaxKind.IdentifierToken, "C");
    }
    N(SyntaxKind.EndOfFileToken);
}
EOF();
"#;
        let t = parse_expected_tree_from_dsl(s).unwrap();
        assert_eq!(t.root.kind, "CompilationUnit");
        assert!(t.root.children.iter().any(|n| n.kind == "ClassDeclaration"));
    }
}
