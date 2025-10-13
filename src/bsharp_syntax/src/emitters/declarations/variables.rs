use std::fmt::Write;
use crate::declarations::local_variable_declaration::VariableDeclarator;
use crate::declarations::LocalVariableDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for VariableDeclarator {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for LocalVariableDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}