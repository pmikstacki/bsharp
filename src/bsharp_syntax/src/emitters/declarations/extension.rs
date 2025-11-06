use crate::declarations::ExtensionDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for ExtensionDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("extension ")?;
        self.receiver.emit(w, cx)?;
        cx.open_brace(w)?;
        for (i, m) in self.members.iter().enumerate() {
            if i > 0 {
                cx.between_top_level_declarations(w)?;
            }
            cx.write_indent(w)?;
            m.emit(w, cx)?;
            cx.nl(w)?;
        }
        cx.close_brace(w)?;
        Ok(())
    }
}
