#[derive(Debug, Clone)]
pub struct MatchSpec {
    pub roslyn_kind: String,
    pub our_kind: String,
    pub token_field: Option<String>,
}

#[allow(dead_code)]
pub fn map_kind(roslyn_kind: &str) -> Option<MatchSpec> {
    // Minimal initial mapping for pilot; expand incrementally.
    match roslyn_kind {
        "CompilationUnit" => Some(MatchSpec { roslyn_kind: roslyn_kind.into(), our_kind: "CompilationUnit".into(), token_field: None }),
        _ => None,
    }
}
