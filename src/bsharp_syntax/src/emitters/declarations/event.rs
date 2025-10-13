use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{EventAccessor, EventAccessorList, EventDeclaration};

impl Emit for EventAccessor {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
impl Emit for EventAccessorList {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
impl Emit for EventDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
