use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::FieldDeclaration;

impl Emit for FieldDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope(format!("Field({})", self.name));
        for (i, m) in self.modifiers.iter().enumerate() { if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        self.field_type.emit(w, cx)?; w.write_char(' ')?; write!(w, "{}", self.name)?;
        if let Some(init) = &self.initializer { w.write_str(" = ")?; init.emit(w, cx)?; }
        w.write_char(';')?;
        Ok(())
    }
}
