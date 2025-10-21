use std::fmt::{self, Write};
use std::io::{Write as IoWrite};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::emitters::policy;
use serde::Serialize;

#[derive(Debug)]
pub struct EmitError(pub fmt::Error);

impl From<fmt::Error> for EmitError {
    fn from(e: fmt::Error) -> Self {
        EmitError(e)
    }
}

#[derive(Default)]
pub struct EmitCtx {
    pub indent: usize,
    // Formatting policy flags (lightweight; not exhaustive)
    pub policy_blank_line_between_members: bool,
    // Instrumentation
    pub instrument: bool,
    pub node_stack: Vec<String>,
    pub trace: Option<Box<dyn IoWrite>>, // JSONL writer
    tail: VecDeque<u8>, // ring buffer of recent output
}

impl EmitCtx {
    pub fn new() -> Self {
        Self {
            indent: 0,
            policy_blank_line_between_members: true,
            instrument: false,
            node_stack: Vec::new(),
            trace: None,
            tail: VecDeque::with_capacity(128),
        }
    }

    pub fn push_indent(&mut self) { self.indent += 1; }

    pub fn pop_indent(&mut self) { if self.indent > 0 { self.indent -= 1; } }

    pub fn write_indent<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        for _ in 0..self.indent { w.write_str("    ")?; }
        if self.instrument { self.push_tail_bytes(&vec![b' '; 4 * self.indent]); self.log_action("write_indent", &[("spaces", (4 * self.indent).to_string())]); }
        Ok(())
    }

    pub fn nl<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        w.write_char('\n')?;
        if self.instrument { self.push_tail_bytes(b"\n"); self.log_action("newline", &[]); }
        Ok(())
    }

    pub fn ln<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        self.nl(w)?;
        self.write_indent(w)
    }

    pub fn space<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        w.write_char(' ')?;
        if self.instrument { self.push_tail_bytes(b" "); self.log_action("space", &[]); }
        Ok(())
    }

    pub fn token<W: Write>(&mut self, w: &mut W, s: &str) -> Result<(), EmitError> {
        w.write_str(s)?;
        if self.instrument { self.push_tail_bytes(s.as_bytes()); self.log_action("token", &[("value", s.to_string())]); }
        Ok(())
    }

    // Formatting policy convenience helpers
    pub fn between_header_and_body_of_file<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        // safe to pass &mut self as EmitCtx; policy does not mutate indent
        if self.instrument { self.log_action("policy", &[("name", "between_header_and_body_of_file".to_string())]); }
        policy::between_header_and_body_of_file(&mut self.clone_for_policy(), w)
    }

    pub fn after_file_scoped_namespace_header<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        if self.instrument { self.log_action("policy", &[("name", "after_file_scoped_namespace_header".to_string())]); }
        policy::after_file_scoped_namespace_header(&mut self.clone_for_policy(), w)
    }

    pub fn between_using_blocks_and_declarations<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        if self.instrument { self.log_action("policy", &[("name", "between_using_blocks_and_declarations".to_string())]); }
        policy::between_using_blocks_and_declarations(&mut self.clone_for_policy(), w)
    }

    pub fn between_top_level_declarations<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        if self.instrument { self.log_action("policy", &[("name", "between_top_level_declarations".to_string())]); }
        policy::between_top_level_declarations(&mut self.clone_for_policy(), w)
    }

    pub fn between_members<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        if !self.policy_blank_line_between_members { return Ok(()); }
        if self.instrument { self.log_action("policy", &[("name", "between_members".to_string()), ("blank_line_between_members", self.policy_blank_line_between_members.to_string())]); }
        policy::between_members(&mut self.clone_for_policy(), w)
    }

    // Internal: provide a temporary mutable view for policy functions
    fn clone_for_policy(&self) -> EmitCtx { EmitCtx { indent: self.indent, policy_blank_line_between_members: self.policy_blank_line_between_members, instrument: self.instrument, node_stack: self.node_stack.clone(), trace: None, tail: VecDeque::with_capacity(128) } }

    pub fn join<W, T, F>(&self, w: &mut W, items: &[T], sep: &str, mut f: F) -> Result<(), EmitError>
    where
        W: Write,
        F: FnMut(&T, &mut W) -> Result<(), EmitError>,
    {
        for (i, item) in items.iter().enumerate() {
            if i != 0 { w.write_str(sep)?; }
            f(item, w)?;
        }
        Ok(())
    }

    pub fn bracketed<W, F>(&mut self, w: &mut W, open: char, close: char, mut inner: F) -> Result<(), EmitError>
    where
        W: Write,
        F: FnMut(&mut W) -> Result<(), EmitError>,
    {
        w.write_char(open)?;
        if self.instrument { self.push_tail_bytes(&[open as u8]); self.log_action("token", &[("value", open.to_string())]); }
        inner(w)?;
        w.write_char(close)?;
        if self.instrument { self.push_tail_bytes(&[close as u8]); self.log_action("token", &[("value", close.to_string())]); }
        Ok(())
    }

    pub fn open_brace<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        let indent_before = self.indent;
        w.write_char('{')?;
        if self.instrument { self.push_tail_bytes(b"{"); }
        self.nl(w)?;
        self.push_indent();
        if self.instrument { self.log_action("open_brace", &[("indent_before", indent_before.to_string()), ("indent_after", self.indent.to_string())]); }
        Ok(())
    }

    pub fn close_brace<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        let indent_after_pop = self.indent.saturating_sub(1);
        self.pop_indent();
        self.write_indent(w)?;
        w.write_char('}')?;
        if self.instrument { self.push_tail_bytes(b"}"); self.log_action("close_brace", &[("indent_after_pop", indent_after_pop.to_string())]); }
        Ok(())
    }

    // Instrumentation helpers
    pub fn enter_node<S: Into<String>>(&mut self, name: S) {
        if !self.instrument { return; }
        let n = name.into();
        self.node_stack.push(n.clone());
        self.log_action("enter_node", &[("name", n)]);
    }

    pub fn exit_node(&mut self) {
        if !self.instrument { return; }
        let name = self.node_stack.pop().unwrap_or_default();
        self.log_action("exit_node", &[("name", name)]);
    }

    fn push_tail_bytes(&mut self, bytes: &[u8]) {
        if !self.instrument { return; }
        let cap = 128usize;
        for &b in bytes {
            if self.tail.len() == cap { self.tail.pop_front(); }
            self.tail.push_back(b);
        }
    }

    fn snapshot_tail(&self) -> String {
        if !self.instrument { return String::new(); }
        let mut v = Vec::with_capacity(self.tail.len());
        for &b in &self.tail { v.push(b); }
        let s = String::from_utf8_lossy(&v);
        // Truncate to ~80 chars for brevity
        let tail = s.as_ref();
        let max = 80usize;
        if tail.len() > max { tail[tail.len()-max..].to_string() } else { tail.to_string() }
    }

    pub fn trace_event(&mut self, action: &str, details: &[(&str, String)]) { self.log_action(action, details); }

    fn log_action(&mut self, action: &str, details: &[(&str, String)]) {
        if !self.instrument { return; }
        #[derive(Serialize)]
        struct Event {
            ts: u128,
            node_path: Vec<String>,
            action: String,
            indent: usize,
            details: serde_json::Map<String, serde_json::Value>,
            tail: String,
        }

        let ts = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
        let mut det = serde_json::Map::new();
        for (k, v) in details { det.insert((*k).to_string(), serde_json::Value::String(v.clone())); }
        let tail = self.snapshot_tail();
        let node_path: Vec<String> = self.node_stack.clone();
        let ev = Event { ts, node_path, action: action.to_string(), indent: self.indent, details: det, tail };
        if let Some(w) = self.trace.as_mut() {
            if let Ok(line) = serde_json::to_string(&ev) {
                let _ = w.write_all(line.as_bytes());
                let _ = w.write_all(b"\n");
                let _ = w.flush();
            }
        }
    }
}

pub trait Emit {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError>;
}

pub struct Emitter;

impl Default for Emitter {
    fn default() -> Self {
        Self::new()
    }
}

impl Emitter {
    pub fn new() -> Self {
        Self
    }

    pub fn write<T: Emit>(&self, item: &T) -> Result<String, EmitError> {
        let mut cx = EmitCtx::new();
        self.write_with_ctx(item, &mut cx)
    }

    pub fn write_with_ctx<T: Emit>(&self, item: &T, cx: &mut EmitCtx) -> Result<String, EmitError> {
        let mut s = String::new();
        // Always use instrumented writer; it is a thin wrapper and only logs when cx.instrument is true
        let mut iw = InstrumentedWriter { out: &mut s, cx: cx as *mut EmitCtx };
        item.emit(&mut iw, cx)?;
        Ok(s)
    }
}

/// RAII guard for node-scoped logging. Holds a raw pointer to avoid borrow conflicts.
pub struct NodeScope {
    cx: *mut EmitCtx,
    active: bool,
}

impl NodeScope {
    pub fn new(cx: &mut EmitCtx, name: impl Into<String>) -> Self {
        let ptr = cx as *mut EmitCtx;
        // Immediate call; we don't hold a Rust borrow beyond this function.
        unsafe { (*ptr).enter_node(name); }
        Self { cx: ptr, active: true }
    }
}

impl Drop for NodeScope {
    fn drop(&mut self) {
        if self.active {
            unsafe { (*self.cx).exit_node(); }
        }
    }
}

impl EmitCtx {
    /// Convenience to create a NodeScope without binding type parameters explicitly.
    pub fn node_scope(&mut self, name: impl Into<String>) -> NodeScope {
        NodeScope::new(self, name)
    }
}

struct InstrumentedWriter<'a> {
    out: &'a mut String,
    cx: *mut EmitCtx,
}

impl fmt::Write for InstrumentedWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.out.push_str(s);
        unsafe {
            let ctx = &mut *self.cx;
            if ctx.instrument {
                ctx.push_tail_bytes(s.as_bytes());
                ctx.log_action("write_str", &[("len", s.len().to_string())]);
            }
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.out.push(c);
        unsafe {
            let ctx = &mut *self.cx;
            if ctx.instrument {
                let mut buf = [0u8; 4];
                let s = c.encode_utf8(&mut buf);
                ctx.push_tail_bytes(s.as_bytes());
                ctx.log_action("write_char", &[("ch", c.to_string())]);
            }
        }
        Ok(())
    }
}
