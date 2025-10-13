use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::enum_declaration::EnumMember;
use crate::declarations::EnumDeclaration;

impl Emit for EnumDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
impl Emit for EnumMember {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
