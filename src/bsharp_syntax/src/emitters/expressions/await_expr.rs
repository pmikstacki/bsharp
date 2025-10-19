use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::AwaitExpression;

impl Emit for AwaitExpression {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("await ")?;
        self.expr.emit(w, cx)
    }
}
