use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::ThrowExpression;

impl Emit for ThrowExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("throw")?;
        if let Some(e) = &self.expr {
            w.write_char(' ')?;
            e.emit(w, cx)?;
        }
        Ok(())
    }
}
