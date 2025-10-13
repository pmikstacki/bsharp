use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::RecordDeclaration;

impl Emit for RecordDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
