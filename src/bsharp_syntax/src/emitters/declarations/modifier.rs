use std::fmt::Write;
use crate::declarations::Modifier;
use crate::declarations::modifier::{ModifierCategory, ModifierInfo};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for Modifier {
    fn emit<W: Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        let s = match self {
            Modifier::Public => "public",
            Modifier::Private => "private",
            Modifier::Protected => "protected",
            Modifier::Internal => "internal",
            Modifier::File => "file",
            Modifier::Static => "static",
            Modifier::Abstract => "abstract",
            Modifier::Sealed => "sealed",
            Modifier::Virtual => "virtual",
            Modifier::Override => "override",
            Modifier::Readonly => "readonly",
            Modifier::Volatile => "volatile",
            Modifier::Const => "const",
            Modifier::Unsafe => "unsafe",
            Modifier::Extern => "extern",
            Modifier::New => "new",
            Modifier::Partial => "partial",
            Modifier::Async => "async",
            Modifier::Required => "required",
            Modifier::Ref => "ref",
            Modifier::Out => "out",
            Modifier::In => "in",
            Modifier::Params => "params",
            Modifier::Fixed => "fixed",
        };
        w.write_str(s)?; Ok(())
    }
}

impl Emit for ModifierCategory{
    fn emit<W: Write>(&self, _w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { Ok(()) }
}

impl Emit for ModifierInfo{
    fn emit<W: Write>(&self, _w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { Ok(()) }
}