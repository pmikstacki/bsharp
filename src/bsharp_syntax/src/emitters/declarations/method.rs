use std::fmt::Write;
use crate::declarations::MethodDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for MethodDeclaration{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Modifiers
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        // Return type and name
        self.return_type.emit(w, cx)?; w.write_char(' ')?; write!(w, "{}", self.name)?;
        // Type parameters
        if let Some(tps) = &self.type_parameters { w.write_char('<')?; for (i,tp) in tps.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } tp.emit(w, cx)?; } w.write_char('>')?; }
        // Parameters
        w.write_char('(')?; for (i,p) in self.parameters.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; } w.write_char(')')?;
        if let Some(cs) = &self.constraints { for c in cs { w.write_char(' ')?; c.emit(w, cx)?; } }
        if let Some(body) = &self.body {
            cx.nl(w)?;
            cx.write_indent(w)?;
            body.emit(w, cx)
        } else { w.write_char(';')?; Ok(()) }
    }
}
