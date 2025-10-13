use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{ClassBodyDeclaration, ClassDeclaration};

impl Emit for ClassDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for ClassBodyDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
