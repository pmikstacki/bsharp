use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{
    FileScopedNamespaceDeclaration, NamespaceBodyDeclaration, NamespaceDeclaration,
};

impl Emit for NamespaceDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope(format!("Namespace({})", self.name));

        // namespace <Name>
        write!(w, "namespace {}", self.name)?;
        cx.trace_event("header_done", &[("has_body", "true".to_string()), ("allman", "true".to_string())]);
        cx.nl(w)?;
        cx.write_indent(w)?;
        cx.trace_event("before_open_brace", &[("has_body", "true".to_string()), ("allman", "true".to_string())]);
        cx.open_brace(w)?;

        // Usings inside namespace
        let mut wrote_any = false;
        for u in &self.using_directives {
            cx.write_indent(w)?;
            u.emit(w, cx)?;
            cx.nl(w)?;
            wrote_any = true;
        }
        if wrote_any && !self.declarations.is_empty() {
            cx.between_using_blocks_and_declarations(w)?; // blank line between usings and declarations
        }

        // Declarations inside namespace
        let mut first = true;
        for d in &self.declarations {
            if !first { cx.between_top_level_declarations(w)?; }
            cx.write_indent(w)?;
            d.emit(w, cx)?;
            cx.nl(w)?;
            first = false;
        }
        // Ensure the closing brace of the namespace is on its own line
        cx.close_brace(w)?;
        Ok(())
    }
}

impl Emit for NamespaceBodyDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            NamespaceBodyDeclaration::Namespace(n) => n.emit(w, cx),
            NamespaceBodyDeclaration::Class(c) => c.emit(w, cx),
            NamespaceBodyDeclaration::Struct(s) => s.emit(w, cx),
            NamespaceBodyDeclaration::Interface(i) => i.emit(w, cx),
            NamespaceBodyDeclaration::Enum(e) => e.emit(w, cx),
            NamespaceBodyDeclaration::Delegate(d) => d.emit(w, cx),
            NamespaceBodyDeclaration::Record(r) => r.emit(w, cx),
            NamespaceBodyDeclaration::GlobalAttribute(ga) => ga.emit(w, cx),
        }
    }
}

impl Emit for FileScopedNamespaceDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        // Only emit the header here; the CompilationUnit emitter will emit inner content
        write!(w, "namespace {};", self.name)?;
        Ok(())
    }
}
