use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::enum_declaration::EnumMember;
use crate::declarations::EnumDeclaration;

impl Emit for EnumDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Attributes
        for (i, al) in self.attributes.iter().enumerate() { if i != 0 { cx.write_indent(w)?; } al.emit(w, cx)?; cx.nl(w)?; }
        // Header
        if !self.attributes.is_empty() { cx.write_indent(w)?; }
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        w.write_str("enum ")?; write!(w, "{}", self.name)?;
        if let Some(ut) = &self.underlying_type { w.write_str(" : ")?; ut.emit(w, cx)?; }
        cx.nl(w)?;
        cx.write_indent(w)?;
        cx.open_brace(w)?;
        for (i, m) in self.enum_members.iter().enumerate() {
            cx.write_indent(w)?;
            m.emit(w, cx)?;
            if i + 1 != self.enum_members.len() { w.write_char(',')?; }
            cx.nl(w)?;
        }
        cx.close_brace(w)?;
        Ok(())
    }
}
impl Emit for EnumMember {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        for al in &self.attributes { al.emit(w, cx)?; w.write_char(' ')?; }
        write!(w, "{}", self.name)?;
        if let Some(v) = &self.value { w.write_str(" = ")?; v.emit(w, cx)?; }
        Ok(())
    }
}
