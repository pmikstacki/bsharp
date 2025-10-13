use std::fmt::Write;
use crate::declarations::{MemberBody, MemberDeclaration};
use crate::expressions::{AnonymousObjectMember, MemberAccessExpression};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for MemberBody{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for MemberDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for MemberAccessExpression {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for AnonymousObjectMember{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}