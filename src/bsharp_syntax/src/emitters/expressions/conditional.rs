use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::ConditionalExpression;

impl Emit for ConditionalExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.condition.emit(w, cx)?;
        cx.space(w)?;
        w.write_str("?")?;
        cx.space(w)?;
        self.consequence.emit(w, cx)?;
        cx.space(w)?;
        w.write_str(":")?;
        cx.space(w)?;
        self.alternative.emit(w, cx)
    }
}
