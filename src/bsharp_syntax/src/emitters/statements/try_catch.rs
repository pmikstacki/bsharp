use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{TryStatement, CatchClause, FinallyClause};

impl Emit for TryStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("try")?;
        cx.nl(w)?; cx.write_indent(w)?; self.try_block.emit(w, cx)?;
        for c in &self.catches { cx.nl(w)?; cx.write_indent(w)?; c.emit(w, cx)?; }
        if let Some(f) = &self.finally_clause { cx.nl(w)?; cx.write_indent(w)?; f.emit(w, cx)?; }
        Ok(())
    }
}

impl Emit for CatchClause {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("catch")?;
        if self.exception_type.is_some() || self.exception_variable.is_some() {
            w.write_char(' ')?;
            w.write_char('(')?;
            if let Some(ty) = &self.exception_type { write!(w, "{}", ty)?; }
            if let Some(var) = &self.exception_variable { if self.exception_type.is_some() { w.write_char(' ')?; } write!(w, "{}", var)?; }
            w.write_char(')')?;
        }
        if let Some(when) = &self.when_clause { w.write_str(" when (")?; when.emit(w, cx)?; w.write_char(')')?; }
        cx.nl(w)?; cx.write_indent(w)?; self.block.emit(w, cx)
    }
}

impl Emit for FinallyClause {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        w.write_str("finally")?;
        cx.nl(w)?; cx.write_indent(w)?; self.block.emit(w, cx)
    }
}
