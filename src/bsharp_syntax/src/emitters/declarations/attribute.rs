use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use std::fmt::Write as _;
use crate::declarations::attribute::AttributeName;
use crate::declarations::{Attribute, AttributeList, GlobalAttribute};

impl Emit for AttributeList {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        w.write_char('[')?;
        for (i, a) in self.attributes.iter().enumerate() {
            if i != 0 {
                w.write_str(", ")?;
            }
            write!(w, "{}", a.name)?;
        }
        w.write_char(']')?;
        Ok(())
    }
}

impl Emit for Attribute {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        w.write_char('[')?;
        write!(w, "{}", self.name)?;
        w.write_char(']')?;
        Ok(())
    }
}

impl Emit for GlobalAttribute {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for AttributeName {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
