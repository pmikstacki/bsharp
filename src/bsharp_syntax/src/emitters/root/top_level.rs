use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::ast::TopLevelDeclaration;

impl Emit for TopLevelDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            TopLevelDeclaration::Namespace(n) => n.emit(w, cx),
            TopLevelDeclaration::FileScopedNamespace(n) => n.emit(w, cx),
            TopLevelDeclaration::Class(c) => c.emit(w, cx),
            TopLevelDeclaration::Struct(s) => s.emit(w, cx),
            TopLevelDeclaration::Record(r) => r.emit(w, cx),
            TopLevelDeclaration::Interface(i) => i.emit(w, cx),
            TopLevelDeclaration::Enum(e) => e.emit(w, cx),
            TopLevelDeclaration::Delegate(d) => d.emit(w, cx),
            TopLevelDeclaration::GlobalAttribute(ga) => ga.emit(w, cx),
        }
    }
}
