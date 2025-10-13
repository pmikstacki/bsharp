use std::fmt::{self, Write};

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
}

impl EmitCtx {
    pub fn new() -> Self {
        Self { indent: 0 }
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
        let mut s = String::new();
        let mut cx = EmitCtx::new();
        item.emit(&mut s, &mut cx)?;
        Ok(s)
    }
}
