use crate::{DiagnosticSeverity, SourceLocation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    // Control flow smell thresholds
    pub cf_high_complexity_threshold: usize, // default: 10
    pub cf_deep_nesting_threshold: usize,    // default: 4

    // Phase 1: config toggles and severities
    #[serde(default)]
    pub enable_rulesets: HashMap<String, bool>,
    #[serde(default)]
    pub enable_passes: HashMap<String, bool>,
    #[serde(default)]
    pub rule_severities: HashMap<String, DiagnosticSeverity>, // keyed by DiagnosticCode.as_str()

    // Workspace filters (CLI can map flags here)
    #[serde(default)]
    pub workspace: WorkspaceConfig,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    #[serde(default)]
    pub follow_refs: bool,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AnalysisContext {
    file: String,
    source: String,
    line_starts: Vec<usize>,
    pub config: AnalysisConfig,
}

impl AnalysisContext {
    /// Create a context for a single file with its full source contents.
    pub fn new(file: impl Into<String>, source: impl Into<String>) -> Self {
        let file = file.into();
        let source = source.into();
        let line_starts = Self::compute_line_starts(&source);
        let mut ctx = Self {
            file,
            source,
            line_starts,
            config: AnalysisConfig::default(),
        };
        // Initialize default thresholds
        ctx.config.cf_high_complexity_threshold = 10;
        ctx.config.cf_deep_nesting_threshold = 4;
        // Defaults for new fields
        ctx.config.workspace.follow_refs = true;
        ctx
    }

    fn compute_line_starts(src: &str) -> Vec<usize> {
        // 0-based byte offsets where a new line starts
        let mut starts = vec![0usize];
        for (i, b) in src.as_bytes().iter().enumerate() {
            if *b == b'\n' {
                // Next line starts after this newline
                starts.push(i + 1);
            }
        }
        starts
    }

    /// Map a byte span (start, length) to a SourceLocation (1-based line/column).
    /// If the span exceeds the source length, it will clamp safely.
    pub fn location_from_span(&self, start: usize, length: usize) -> SourceLocation {
        let start = start.min(self.source.len());
        let length = length.min(self.source.len().saturating_sub(start));

        // Binary search for the line containing start
        let line_idx = match self.line_starts.binary_search(&start) {
            Ok(idx) => idx,
            Err(idx) => idx.saturating_sub(1),
        };
        let line_start = *self.line_starts.get(line_idx).unwrap_or(&0);
        let column0 = start.saturating_sub(line_start);

        SourceLocation {
            file: self.file.clone(),
            line: line_idx + 1,  // Convert to 1-based
            column: column0 + 1, // Convert to 1-based
            length,
        }
    }

    /// Convenience to get a SourceLocation from a (start, end) range.
    pub fn location_from_range(&self, start: usize, end: usize) -> SourceLocation {
        let length = end.saturating_sub(start);
        self.location_from_span(start, length)
    }

    /// Get the full source text (read-only)
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Get the file path associated with this analysis context.
    pub fn file_path(&self) -> &str {
        &self.file
    }

    /// Get the text of a 1-based line number. Returns an empty string if out of range.
    pub fn line_text(&self, line: usize) -> &str {
        if line == 0 {
            return "";
        }
        let idx = line - 1;
        let start = *self.line_starts.get(idx).unwrap_or(&self.source.len());
        let end = *self.line_starts.get(idx + 1).unwrap_or(&self.source.len());
        let slice = &self.source[start..end];
        // Trim trailing newline for cleaner display
        slice.strip_suffix('\n').unwrap_or(slice)
    }
}
