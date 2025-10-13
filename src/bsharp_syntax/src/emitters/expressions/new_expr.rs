use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::NewExpression;

impl Emit for NewExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
