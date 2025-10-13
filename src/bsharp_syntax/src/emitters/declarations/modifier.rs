use std::fmt::Write;
use crate::declarations::Modifier;
use crate::declarations::modifier::{ModifierCategory, ModifierInfo};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for Modifier {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for ModifierCategory{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for ModifierInfo{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}