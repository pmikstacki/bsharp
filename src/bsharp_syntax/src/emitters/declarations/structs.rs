use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{StructBodyDeclaration, StructDeclaration};

impl Emit for StructDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}

impl Emit for StructBodyDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
