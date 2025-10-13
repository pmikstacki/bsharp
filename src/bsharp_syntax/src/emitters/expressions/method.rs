use std::fmt::Write;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::AnonymousMethodExpression;

impl Emit for AnonymousMethodExpression {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}