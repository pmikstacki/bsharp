use crate::declarations::attribute::AttributeName;
use crate::declarations::{Attribute, AttributeList, GlobalAttribute};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use std::fmt::Write as _;

impl Emit for AttributeList {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        w.write_char('[')?;
        for (i, a) in self.attributes.iter().enumerate() {
            if i != 0 {
                w.write_str(", ")?;
            }
            a.emit(w, cx)?;
        }
        w.write_char(']')?;
        Ok(())
    }
}

impl Emit for Attribute {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        // Name: either structured (qualified) or simple identifier
        if let Some(structured) = &self.structured {
            structured.emit(w, cx)?;
        } else {
            write!(w, "{}", self.name)?;
        }
        // Arguments: (arg0, arg1, ...)
        if !self.arguments.is_empty() {
            w.write_char('(')?;
            for (i, arg) in self.arguments.iter().enumerate() {
                if i != 0 {
                    w.write_str(", ")?;
                }
                arg.emit(w, cx)?;
            }
            w.write_char(')')?;
        }
        Ok(())
    }
}

impl Emit for GlobalAttribute {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        // Syntax: [target: Attribute]
        w.write_char('[')?;
        write!(w, "{}: ", self.target)?;
        // Reuse Attribute emission but without outer brackets; our Attribute.emit emits only name(and args) now
        self.attribute.emit(w, cx)?;
        w.write_char(']')?;
        Ok(())
    }
}

impl Emit for AttributeName {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        for (i, q) in self.qualifier.iter().enumerate() {
            if i != 0 {
                w.write_char('.')?;
            }
            write!(w, "{}", q)?;
        }
        if !self.qualifier.is_empty() {
            w.write_char('.')?;
        }
        write!(w, "{}", self.name)?;
        if !self.type_arguments.is_empty() {
            w.write_char('<')?;
            for (i, t) in self.type_arguments.iter().enumerate() {
                if i != 0 {
                    w.write_str(", ")?;
                }
                write!(w, "{}", t)?;
            }
            w.write_char('>')?;
        }
        Ok(())
    }
}
