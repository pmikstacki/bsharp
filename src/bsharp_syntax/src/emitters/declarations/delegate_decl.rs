use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::DelegateDeclaration;

impl Emit for DelegateDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        for al in &self.attributes { cx.write_indent(w)?; al.emit(w, cx)?; cx.nl(w)?; }
        cx.write_indent(w)?;
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        w.write_str("delegate ")?;
        self.return_type.emit(w, cx)?; w.write_char(' ')?; write!(w, "{}", self.name)?;
        if !self.type_parameters.is_empty() { w.write_char('<')?; for (i,tp) in self.type_parameters.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } tp.emit(w, cx)?; } w.write_char('>')?; }
        w.write_char('(')?; for (i,p) in self.parameters.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; } w.write_char(')')?;
        if let Some(cs) = &self.constraints { for c in cs { w.write_char(' ')?; c.emit(w, cx)?; } }
        w.write_char(';')?; Ok(())
    }
}
