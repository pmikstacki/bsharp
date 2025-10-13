use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::InvocationExpression;

impl Emit for InvocationExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
