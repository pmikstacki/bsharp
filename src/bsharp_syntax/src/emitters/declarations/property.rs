use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{PropertyAccessor, PropertyDeclaration};

impl Emit for PropertyAccessor {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}

impl Emit for PropertyDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
