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
    let mut out = Vec::new();
    for (idx, _) in content.match_indices("UsingTree(") {
        let prefix = &content[..idx];
        if let Some(text_pos) = prefix.rfind("var text =") {
            let rest = &content[text_pos..];
            if let Some(qidx) = rest.find('"') {
                let rest2 = &rest[qidx + 1..];
                if let Some(end_rel) = rest2.find("\";") {
                    let src = &rest2[..end_rel];
                    let src_code = src.to_string();
                    out.push(ExtractedStructureTest {
                        method_name: None,
                        src_code,
                        expected: ExpectedTree {
                            root: ExpectedNode { kind: "CompilationUnit".into(), token_value: None, children: vec![] },
                        },
                        call_pos: idx,
                    });
                }
            }
        }
    }
    out
}
