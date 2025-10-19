use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::MemberAccessExpression;

impl Emit for MemberAccessExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.object.emit(w, cx)?;
        w.write_str(".")?;
        write!(w, "{}", self.member)?;
        Ok(())
    }
}
