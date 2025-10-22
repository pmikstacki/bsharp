use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::CheckedExpression;

impl Emit for CheckedExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("checked(")?;
        self.expr.emit(w, cx)?;
        w.write_char(')')?;
        Ok(())
    }
}
