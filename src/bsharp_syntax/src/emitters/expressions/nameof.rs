use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::NameofExpression;

impl Emit for NameofExpression {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("nameof(")?;
        self.expr.emit(w, cx)?;
        w.write_char(')')?;
        Ok(())
    }
}
