use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::AssignmentExpression;

impl Emit for AssignmentExpression {
    fn emit<W: std::fmt::Write>(&self, _w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
