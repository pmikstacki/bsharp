use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::DestructorDeclaration;

impl Emit for DestructorDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Attributes are Attribute, wrap each in [] on its own line
        for a in &self.attributes { cx.write_indent(w)?; w.write_char('[')?; a.emit(w, cx)?; w.write_char(']')?; cx.nl(w)?; }
        // Modifiers and signature
        cx.write_indent(w)?;
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        w.write_char('~')?; write!(w, "{}", self.name)?; w.write_str("()")?;
        if let Some(body) = &self.body { cx.nl(w)?; cx.write_indent(w)?; body.emit(w, cx) } else { w.write_char(';')?; Ok(()) }
    }
}
