#![allow(dead_code)]
use super::FormatOptions;

/// Internal writer that normalizes newlines and blank lines, and trims trailing whitespace.
/// Not yet wired into emitters; introduced for future integration.
pub(crate) struct FmtWriter {
    buf: String,
    opts: FormatOptions,
    last_line_start: usize,
    at_line_start: bool,
    consecutive_blank_lines: u8,
}

impl FmtWriter {
    pub(crate) fn new(opts: FormatOptions) -> Self {
        Self {
            buf: String::new(),
            opts,
            last_line_start: 0,
            at_line_start: true,
            consecutive_blank_lines: 0,
        }
    }

    pub(crate) fn write_str(&mut self, s: &str) {
        self.buf.push_str(s);
        self.at_line_start = false;
    }

    pub(crate) fn write_char(&mut self, ch: char) {
        self.buf.push(ch);
        if ch == '\n' {
            // External newlines should rarely be written; prefer newline().
            self.after_newline_side_effects();
        } else {
            self.at_line_start = false;
        }
    }

    fn trim_trailing_ws_in_current_line(&mut self) {
        // Trim spaces/tabs between last_line_start and current end
        let mut end = self.buf.len();
        while end > self.last_line_start {
            let c = self.buf.as_bytes()[end - 1];
            if c == b' ' || c == b'\t' { end -= 1; } else { break; }
        }
        if end < self.buf.len() { self.buf.truncate(end); }
    }

    fn after_newline_side_effects(&mut self) {
        // We've just written a newline into buf; update counters/state.
        self.last_line_start = self.buf.len();
        self.at_line_start = true;
        self.consecutive_blank_lines += 1;
        if self.consecutive_blank_lines > self.opts.max_consecutive_blank_lines {
            // Remove the last newline if we exceeded maximum
            let nl_len = self.opts.newline.len();
            if self.buf.len() >= nl_len { self.buf.truncate(self.buf.len() - nl_len); }
            // Do not advance state further
            self.consecutive_blank_lines = self.opts.max_consecutive_blank_lines;
            self.at_line_start = true;
            self.last_line_start = self.buf.len();
        }
    }

    pub(crate) fn newline(&mut self) {
        // Trim trailing whitespace and determine if current line is blank after trimming
        self.trim_trailing_ws_in_current_line();
        let was_blank = self.buf.len() == self.last_line_start;
        self.buf.push_str(self.opts.newline);
        self.after_newline_side_effects();
        if !was_blank { self.consecutive_blank_lines = 0; }
    }

    /// Ensure exactly one blank line between sections.
    pub(crate) fn ensure_blank_line(&mut self) {
        // Ensure current line ends with newline
        if !self.at_line_start { self.newline(); }
        // If we already have a blank line, nothing to do
        if self.consecutive_blank_lines >= 1 { return; }
        // Insert exactly one more newline to create a blank line
        self.newline();
    }

    /// Allow at most one blank line.
    pub(crate) fn maybe_blank_line(&mut self) {
        if !self.at_line_start { self.newline(); }
        if self.consecutive_blank_lines == 0 { self.newline(); }
    }

    pub(crate) fn finalize(mut self) -> String {
        if self.opts.trim_trailing_whitespace {
            self.trim_trailing_ws_in_current_line();
        }
        // Strip all trailing whitespace (spaces, tabs, CR, LF) at EOF
        while let Some(&b) = self.buf.as_bytes().last() {
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                self.buf.pop();
            } else { break; }
        }
        if self.opts.ensure_final_newline {
            let nl = self.opts.newline;
            self.buf.push_str(nl);
            // Collapse any extra blank line before EOF: ensure there is only a single final newline
            // If we ended up with two consecutive newlines at the end, remove the extra one(s).
            while self.buf.as_bytes().len() >= 2 {
                let bytes = self.buf.as_bytes();
                let len = bytes.len();
                if bytes[len - 1] == b'\n' && bytes[len - 2] == b'\n' { self.buf.pop(); } else { break; }
            }
        }
        self.buf
    }

    pub(crate) fn into_string(self) -> String { self.buf }
}
