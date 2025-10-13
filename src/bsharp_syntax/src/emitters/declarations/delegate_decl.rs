use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::DelegateDeclaration;

impl Emit for DelegateDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
