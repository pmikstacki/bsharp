use crate::emitters::emit_trait::{EmitError, Emitter, EmitCtx};
use crate::ast::CompilationUnit;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::BufWriter;

mod fmt_writer;
use fmt_writer::FmtWriter;

#[derive(Clone, Debug)]
pub struct FormatOptions {
    pub indent_width: usize,
    pub newline: &'static str,
    pub ensure_final_newline: bool,
    pub max_consecutive_blank_lines: u8,
    pub blank_line_between_members: bool,
    pub trim_trailing_whitespace: bool,
    // Instrumentation options
    pub instrument_emission: bool,
    pub trace_file: Option<PathBuf>,
    pub current_file: Option<PathBuf>,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            indent_width: 4,
            newline: "\n",
            ensure_final_newline: true,
            max_consecutive_blank_lines: 1,
            blank_line_between_members: true,
            trim_trailing_whitespace: true,
            instrument_emission: false,
            trace_file: None,
            current_file: None,
        }
    }
}

pub struct Formatter {
    pub opts: FormatOptions,
}

impl Formatter {
    pub fn new(opts: FormatOptions) -> Self { Self { opts } }

    pub fn format_compilation_unit(&self, cu: &CompilationUnit) -> Result<String, EmitError> {
        // Phase 1: reuse emitters to produce raw output, then normalize via FmtWriter
        let emitter = Emitter::new();
        let mut cx = EmitCtx::new();
        // Drive policy from options
        cx.policy_blank_line_between_members = self.opts.blank_line_between_members;
        // Instrumentation wiring
        if self.opts.instrument_emission {
            cx.instrument = true;
            if let Some(path) = &self.opts.trace_file {
                if let Ok(f) = OpenOptions::new().create(true).append(true).open(path) {
                    cx.trace = Some(Box::new(f));
                }
            } else {
                cx.trace = Some(Box::new(BufWriter::new(std::io::stdout())));
            }
            if let Some(p) = &self.opts.current_file {
                cx.trace_event("session_start", &[("file", p.display().to_string())]);
            } else {
                cx.trace_event("session_start", &[]);
            }
        }
        let raw = emitter.write_with_ctx(cu, &mut cx)?;
        Ok(self.normalize_text(&raw))
    }

    /// NOTE: this is only a WA to make the test pass and formatter work. in the future we need to determine why formatting fails for nested types. ~mikserek
    /// Normalize text-only: newlines, blank lines and trailing whitespace.
    /// Does not perform structural reformatting.
    pub fn normalize_text(&self, raw: &str) -> String {
        let mut fw = FmtWriter::new(self.opts.clone());
        let mut segments: Vec<&str> = raw.split_terminator('\n').collect();
        while matches!(segments.last(), Some(s) if s.trim().is_empty()) { segments.pop(); }
        // Minimal Allman header normalization
        let mut out: Vec<String> = Vec::with_capacity(segments.len());
        for &line in &segments {
            let t = line.trim_end();
            // If the line contains an inline '{', consider splitting it if it looks like a header.
            if let Some(bi) = t.find('{') {
                let (before_raw, after_raw) = t.split_at(bi);
                let before = before_raw.trim_end();
                // Safety: avoid splitting obvious initializers or attributes
                let before_ns = before.trim_start();
                let looks_initializer = before.contains('=') || before.contains(" new ");
                let has_semicolon = before.contains(';');
                if !before.is_empty() && !looks_initializer && !has_semicolon {
                    // Detect headers: namespace/type/method signature (has ')' before '{')
                    let is_namespace = before_ns.starts_with("namespace ");
                    let type_markers = ["class ", "struct ", "interface ", "enum ", "record "];
                    let is_type = type_markers.iter().any(|kw| before_ns.contains(kw));
                    let is_method_like = before.contains(')')
                        && !before_ns.starts_with("switch ")
                        && !before_ns.starts_with("while ")
                        && !before_ns.starts_with("for ")
                        && !before_ns.starts_with("foreach ")
                        && !before_ns.starts_with("if ")
                        && !before_ns.starts_with("else ")
                        && !before_ns.starts_with("try ")
                        && !before_ns.starts_with("catch ")
                        && !before_ns.starts_with("finally ")
                        && !before_ns.starts_with("lock ")
                        && !before_ns.starts_with("using ");
                    if is_namespace || is_type || is_method_like {
                        let lead_ws_len = line.len() - line.trim_start().len();
                        let indent = &line[..lead_ws_len];
                        out.push(before.to_string());
                        // after_raw starts with '{', keep any suffix after it on same brace line
                        let mut brace_line = String::new();
                        brace_line.push_str(indent);
                        brace_line.push('{');
                        let after = &after_raw[1..];
                        if !after.is_empty() { brace_line.push_str(after); }
                        out.push(brace_line);
                        continue;
                    }
                }
            }
            out.push(line.to_string());
        }
        for (i, line) in out.iter().enumerate() {
            if !line.is_empty() { fw.write_str(line); }
            if i + 1 < out.len() { fw.newline(); }
        }
        fw.finalize()
    }
}
