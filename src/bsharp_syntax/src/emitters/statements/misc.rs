use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{CheckedStatement, UncheckedStatement, LockStatement, FixedStatement, UnsafeStatement, LocalFunctionStatement, ForInitializer};

impl Emit for CheckedStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("checked ")?; self.body.emit(w, cx)
    }
}
impl Emit for UncheckedStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("unchecked ")?; self.body.emit(w, cx)
    }
}
impl Emit for LockStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("lock (")?; self.expr.emit(w, cx)?; w.write_str(") ")?; self.body.emit(w, cx)
    }
}
impl Emit for FixedStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("fixed (")?; self.var_type.emit(w, cx)?; w.write_char(' ')?; write!(w, "{} = ", self.var_name)?; self.initializer.emit(w, cx)?; w.write_str(") ")?; self.body.emit(w, cx)
    }
}
impl Emit for UnsafeStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,cx:&mut EmitCtx)->Result<(),EmitError>{ w.write_str("unsafe ")?; self.body.emit(w, cx) }
}
impl Emit for LocalFunctionStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        // modifiers
        for (i, m) in self.modifiers.iter().enumerate() { if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        // return type and name
        self.return_type.emit(w, cx)?; w.write_char(' ')?; write!(w, "{}", self.name)?;
        // type parameters
        if let Some(tps) = &self.type_parameters { w.write_char('<')?; for (i,tp) in tps.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } tp.emit(w, cx)?; } w.write_char('>')?; }
        // parameters
        w.write_char('(')?; for (i,p) in self.parameters.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; } w.write_char(')')?;
        // constraints
        if let Some(cs) = &self.constraints { for c in cs { w.write_char(' ')?; c.emit(w, cx)?; } }
        w.write_char(' ')?; self.body.emit(w, cx)
    }
}
impl Emit for ForInitializer {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        match self {
            ForInitializer::Declaration(d) => d.emit(w, cx),
            ForInitializer::Expressions(exprs) => {
                for (i, e) in exprs.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } e.emit(w, cx)?; }
                Ok(())
            }
        }
    }
}
