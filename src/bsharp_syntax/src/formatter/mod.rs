use crate::emitters::emit_trait::{EmitError, Emitter, EmitCtx};
use crate::ast::CompilationUnit;

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
        let raw = emitter.write_with_ctx(cu, &mut cx)?;
        Ok(self.normalize_text(&raw))
    }

    /// Normalize text-only: newlines, blank lines and trailing whitespace.
    /// Does not perform structural reformatting.
    pub fn normalize_text(&self, raw: &str) -> String {
        let mut fw = FmtWriter::new(self.opts.clone());
        for line in raw.split('\n') {
            if !line.is_empty() { fw.write_str(line); }
            fw.newline();
        }
        fw.finalize()
    }
}

