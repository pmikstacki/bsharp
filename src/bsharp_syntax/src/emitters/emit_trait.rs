use std::fmt::{self, Write};
use crate::emitters::policy;

#[derive(Debug)]
pub struct EmitError(pub fmt::Error);

impl From<fmt::Error> for EmitError {
    fn from(e: fmt::Error) -> Self {
        EmitError(e)
    }
}

#[derive(Default, Clone)]
pub struct EmitCtx {
    pub indent: usize,
    // Formatting policy flags (lightweight; not exhaustive)
    pub policy_blank_line_between_members: bool,
}

impl EmitCtx {
    pub fn new() -> Self {
        Self { indent: 0, policy_blank_line_between_members: true }
    }

    pub fn push_indent(&mut self) { self.indent += 1; }

    pub fn pop_indent(&mut self) { if self.indent > 0 { self.indent -= 1; } }

    pub fn write_indent<W: Write>(&self, w: &mut W) -> Result<(), EmitError> {
        for _ in 0..self.indent { w.write_str("    ")?; }
        Ok(())
    }

    pub fn nl<W: Write>(&self, w: &mut W) -> Result<(), EmitError> {
        w.write_char('\n')?;
        Ok(())
    }

    pub fn ln<W: Write>(&self, w: &mut W) -> Result<(), EmitError> {
        self.nl(w)?;
        self.write_indent(w)
    }

    pub fn space<W: Write>(&self, w: &mut W) -> Result<(), EmitError> {
        w.write_char(' ')?;
        Ok(())
    }

    pub fn token<W: Write>(&self, w: &mut W, s: &str) -> Result<(), EmitError> {
        w.write_str(s)?;
        Ok(())
    }

    // Formatting policy convenience helpers
    pub fn between_header_and_body_of_file<W: Write>(&self, w: &mut W) -> Result<(), EmitError> {
        // safe to pass &mut self as EmitCtx; policy does not mutate indent
        policy::between_header_and_body_of_file(&mut self.clone_for_policy(), w)
    }

    pub fn after_file_scoped_namespace_header<W: Write>(&self, w: &mut W) -> Result<(), EmitError> {
        policy::after_file_scoped_namespace_header(&mut self.clone_for_policy(), w)
    }

    pub fn between_using_blocks_and_declarations<W: Write>(&self, w: &mut W) -> Result<(), EmitError> {
        policy::between_using_blocks_and_declarations(&mut self.clone_for_policy(), w)
    }

    pub fn between_top_level_declarations<W: Write>(&self, w: &mut W) -> Result<(), EmitError> {
        policy::between_top_level_declarations(&mut self.clone_for_policy(), w)
    }

    pub fn between_members<W: Write>(&self, w: &mut W) -> Result<(), EmitError> {
        if !self.policy_blank_line_between_members { return Ok(()); }
        policy::between_members(&mut self.clone_for_policy(), w)
    }

    // Internal: provide a temporary mutable view for policy functions
    fn clone_for_policy(&self) -> EmitCtx { EmitCtx { indent: self.indent, policy_blank_line_between_members: self.policy_blank_line_between_members } }

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

    pub fn bracketed<W, F>(&self, w: &mut W, open: char, close: char, mut inner: F) -> Result<(), EmitError>
    where
        W: Write,
        F: FnMut(&mut W) -> Result<(), EmitError>,
    {
        w.write_char(open)?;
        inner(w)?;
        w.write_char(close)?;
        Ok(())
    }

    pub fn open_brace<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        w.write_char('{')?;
        self.nl(w)?;
        self.push_indent();
        Ok(())
    }

    pub fn close_brace<W: Write>(&mut self, w: &mut W) -> Result<(), EmitError> {
        self.pop_indent();
        self.write_indent(w)?;
        w.write_char('}')?;
        Ok(())
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
        item.emit(&mut s, cx)?;
        Ok(s)
    }
}
