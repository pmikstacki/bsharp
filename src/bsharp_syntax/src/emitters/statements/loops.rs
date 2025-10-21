use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{DoWhileStatement, WhileStatement, ForStatement, ForEachStatement};

impl Emit for DoWhileStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("do")?;
        cx.nl(w)?; cx.write_indent(w)?;
        let is_block = matches!(self.body.as_ref(), crate::statements::statement::Statement::Block(_));
        self.body.emit(w, cx)?;
        if is_block {
            w.write_char(' ')?;
        } else {
            cx.nl(w)?; cx.write_indent(w)?;
        }
        w.write_str("while (")?;
        self.condition.emit(w, cx)?;
        w.write_str(");")?;
        Ok(())
    }
}

impl Emit for WhileStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("while (")?;
        self.condition.emit(w, cx)?;
        w.write_str(")")?;
        cx.nl(w)?; cx.write_indent(w)?; self.body.emit(w, cx)
    }
}

impl Emit for ForStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("for (")?;
        if let Some(init) = &self.initializer { init.emit(w, cx)?; }
        w.write_str("; ")?;
        if let Some(cond) = &self.condition { cond.emit(w, cx)?; }
        w.write_str("; ")?;
        for (i, it) in self.iterator.iter().enumerate() { if i!=0 { w.write_str(", ")?; } it.emit(w, cx)?; }
        w.write_str(")")?;
        cx.nl(w)?; cx.write_indent(w)?; self.body.emit(w, cx)
    }
}

impl Emit for ForEachStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("foreach (")?;
        if self.is_await { w.write_str("await ")?; }
        self.var_type.emit(w, cx)?; w.write_char(' ')?; write!(w, "{} in ", self.var_name)?;
        self.collection.emit(w, cx)?;
        w.write_str(")")?;
        cx.nl(w)?; cx.write_indent(w)?; self.body.emit(w, cx)
    }
}
