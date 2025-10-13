use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::DestructorDeclaration;

impl Emit for DestructorDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
