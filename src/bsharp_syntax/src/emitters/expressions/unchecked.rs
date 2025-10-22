use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::UncheckedExpression;

impl Emit for UncheckedExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("unchecked(")?;
        self.expr.emit(w, cx)?;
        w.write_char(')')?;
        Ok(())
    }
}
