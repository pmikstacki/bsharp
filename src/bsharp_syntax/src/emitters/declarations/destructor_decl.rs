use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::DestructorDeclaration;

impl Emit for DestructorDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Attributes are Attribute, wrap each in [] on its own line
        for a in &self.attributes { cx.write_indent(w)?; w.write_char('[')?; a.emit(w, cx)?; w.write_char(']')?; cx.nl(w)?; }
        // Modifiers and signature
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        w.write_char('~')?; write!(w, "{}", self.name)?; w.write_str("()")?;
        cx.trace_event("header_done", &[("has_body", self.body.is_some().to_string()), ("allman", "true".to_string())]);
        if let Some(body) = &self.body {
            match body {
                crate::statements::statement::Statement::Block(stmts) => {
                    cx.nl(w)?; cx.write_indent(w)?;
                    cx.trace_event("before_open_brace", &[("has_body", "true".to_string()), ("allman", "true".to_string())]);
                    cx.open_brace(w)?;
                    for s in stmts { cx.write_indent(w)?; s.emit(w, cx)?; cx.nl(w)?; }
                    cx.close_brace(w)
                }
                other => { w.write_char(' ')?; other.emit(w, cx) }
            }
        } else { w.write_char(';')?; Ok(()) }
    }
}
