use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::RecordDeclaration;

impl Emit for RecordDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Attributes
        for al in &self.attributes { cx.write_indent(w)?; al.emit(w, cx)?; cx.nl(w)?; }
        // Modifiers and signature
        cx.write_indent(w)?;
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        w.write_str("record")?;
        if self.is_struct { w.write_str(" struct")?; } else { w.write_str(" class")?; }
        w.write_char(' ')?; write!(w, "{}", self.name)?;
        if let Some(params) = &self.parameters { w.write_char('(')?; for (i,p) in params.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; } w.write_char(')')?; }
        if !self.base_types.is_empty() { w.write_str(" : ")?; for (i, bt) in self.base_types.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } bt.emit(w, cx)?; } }
        if let Some(cs) = &self.constraints { for c in cs { w.write_char(' ')?; c.emit(w, cx)?; } }
        cx.nl(w)?;
        cx.write_indent(w)?;
        cx.open_brace(w)?;
        let mut first = true;
        for d in &self.body_declarations {
            if !first { cx.between_members(w)?; }
            cx.write_indent(w)?; d.emit(w, cx)?; cx.nl(w)?;
            first = false;
        }
        cx.close_brace(w)?;
        Ok(())
    }
}

