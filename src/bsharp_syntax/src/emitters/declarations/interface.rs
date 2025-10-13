use std::fmt::Write;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{InterfaceBodyDeclaration, InterfaceDeclaration};

impl Emit for InterfaceDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for InterfaceBodyDeclaration{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
    todo!()
    }

}