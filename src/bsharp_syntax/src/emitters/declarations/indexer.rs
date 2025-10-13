use crate::declarations::{IndexerAccessor, IndexerAccessorList, IndexerDeclaration};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for IndexerAccessor {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for IndexerDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for IndexerAccessorList {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}