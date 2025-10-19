use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::AssignmentExpression;

impl Emit for AssignmentExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.target.emit(w, cx)?;
        cx.space(w)?;
        self.op.emit(w, cx)?;
        cx.space(w)?;
        self.value.emit(w, cx)
    }
}
