use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::AnonymousMethodExpression;
use std::fmt::Write;

impl Emit for AnonymousMethodExpression {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        if self.is_async {
            write!(w, "async ")?;
        }
        write!(w, "delegate(")?;
        for (i, p) in self.parameters.iter().enumerate() {
            if i != 0 {
                write!(w, ", ")?;
            }
            p.emit(w, cx)?;
        }
        write!(w, ") ")?;
        self.body.emit(w, cx)
    }
}
