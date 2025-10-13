use std::fmt::Write;
use crate::declarations::MethodDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for MethodDeclaration{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
