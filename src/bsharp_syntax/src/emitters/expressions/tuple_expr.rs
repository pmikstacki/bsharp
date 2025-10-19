use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{TupleExpression, TupleElement};

impl Emit for TupleExpression {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_char('(')?;
        for (i, el) in self.elements.iter().enumerate() {
            if i != 0 { w.write_str(", ")?; }
            el.emit(w, cx)?;
        }
        w.write_char(')')?;
        Ok(())
    }
}

impl Emit for TupleElement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        if let Some(name) = &self.name { write!(w, "{}: ", name)?; }
        self.value.emit(w, cx)
    }
}
