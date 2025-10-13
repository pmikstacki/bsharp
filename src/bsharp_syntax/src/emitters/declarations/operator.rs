use crate::declarations::OperatorDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use std::fmt::Write;

impl Emit for OperatorDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}


