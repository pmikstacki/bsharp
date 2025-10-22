use crate::ast::CompilationUnit;
use crate::emitters::emit_trait::{EmitCtx, EmitError, Emitter};
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::PathBuf;

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
    pub fn new(opts: FormatOptions) -> Self {
        Self { opts }
    }

    pub fn format_compilation_unit(&self, cu: &CompilationUnit) -> Result<String, EmitError> {
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
        emitter.write_with_ctx(cu, &mut cx)
    }
}
