use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{
    FileScopedNamespaceDeclaration, NamespaceBodyDeclaration, NamespaceDeclaration,
};

impl Emit for NamespaceDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for NamespaceBodyDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for FileScopedNamespaceDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
