use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::TypeDeclaration;

impl Emit for TypeDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
