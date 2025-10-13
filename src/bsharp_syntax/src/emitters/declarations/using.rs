use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{GlobalUsingDirective, UsingDirective};
use crate::statements::UsingStatement;

impl Emit for UsingDirective {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for UsingStatement {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for GlobalUsingDirective {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}
