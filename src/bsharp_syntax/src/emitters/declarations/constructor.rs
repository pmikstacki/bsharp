use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{ConstructorDeclaration, ConstructorInitializer};

impl Emit for ConstructorInitializer {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
impl Emit for ConstructorDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
