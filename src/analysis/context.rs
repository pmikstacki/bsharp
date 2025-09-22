use crate::analysis::diagnostics::SourceLocation;

#[derive(Debug, Default, Clone)]
pub struct AnalysisConfig {
    // Add configuration toggles and thresholds as needed later
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
        Self {
            file,
            source,
            line_starts,
            config: AnalysisConfig::default(),
        }
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
            line: line_idx + 1,           // Convert to 1-based
            column: column0 + 1,           // Convert to 1-based
            length,
        }
    }

    /// Convenience to get a SourceLocation from a (start, end) range.
    pub fn location_from_range(&self, start: usize, end: usize) -> SourceLocation {
        let length = end.saturating_sub(start);
        self.location_from_span(start, length)
    }
}
