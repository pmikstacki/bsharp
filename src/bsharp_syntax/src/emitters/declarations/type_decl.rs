use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::TypeDeclaration;

impl Emit for TypeDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            TypeDeclaration::Class(c) => c.emit(w, cx),
            TypeDeclaration::Struct(s) => s.emit(w, cx),
            TypeDeclaration::Record(r) => r.emit(w, cx),
            TypeDeclaration::Interface(i) => i.emit(w, cx),
            TypeDeclaration::Enum(e) => e.emit(w, cx),
            TypeDeclaration::Delegate(d) => d.emit(w, cx),
        }
    }
}
