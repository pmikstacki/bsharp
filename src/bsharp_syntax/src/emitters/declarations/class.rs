use crate::declarations::{ClassBodyDeclaration, ClassDeclaration};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for ClassDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope(format!("Class({})", self.name));
        // Attributes
        for (i, al) in self.attributes.iter().enumerate() {
            if i != 0 {
                cx.write_indent(w)?;
            }
            al.emit(w, cx)?;
            cx.nl(w)?;
        }
        if !self.attributes.is_empty() {
            cx.write_indent(w)?;
        }
        // Documentation (skip for now)
        // Modifiers
        for (i, m) in self.modifiers.iter().enumerate() {
            if i != 0 {
                cx.space(w)?;
            }
            m.emit(w, cx)?;
        }
        if !self.modifiers.is_empty() {
            cx.space(w)?;
        }
        // Keyword and name
        cx.token(w, "class ")?;
        write!(w, "{}", self.name)?;
        // Type parameters
        if let Some(tps) = &self.type_parameters {
            cx.token(w, "<")?;
            for (i, tp) in tps.iter().enumerate() {
                if i != 0 {
                    cx.token(w, ", ")?;
                }
                tp.emit(w, cx)?;
            }
            cx.token(w, ">")?;
        }
        // Primary constructor parameters (C# 12)
        if let Some(params) = &self.primary_constructor_parameters {
            cx.token(w, "(")?;
            for (i, p) in params.iter().enumerate() {
                if i != 0 {
                    cx.token(w, ", ")?;
                }
                p.emit(w, cx)?;
            }
            cx.token(w, ")")?;
        }
        // Base types
        if !self.base_types.is_empty() {
            cx.token(w, " : ")?;
            for (i, bt) in self.base_types.iter().enumerate() {
                if i != 0 {
                    cx.token(w, ", ")?;
                }
                bt.emit(w, cx)?;
            }
        }
        if let Some(cs) = &self.constraints {
            for c in cs {
                cx.space(w)?;
                c.emit(w, cx)?;
            }
        }
        // Header completed
        cx.trace_event(
            "header_done",
            &[
                ("has_body", "true".to_string()),
                ("allman", "true".to_string()),
            ],
        );
        if self.body_declarations.is_empty() {
            cx.space(w)?;
            w.write_str("{ }")?;
            return Ok(());
        }
        cx.nl(w)?;
        cx.write_indent(w)?;
        // Body choice logging (before open brace)
        cx.trace_event(
            "before_open_brace",
            &[
                ("has_body", "true".to_string()),
                ("allman", "true".to_string()),
            ],
        );
        cx.open_brace(w)?;
        let mut first = true;
        for d in &self.body_declarations {
            if !first {
                cx.between_members(w)?;
            }
            cx.write_indent(w)?;
            d.emit(w, cx)?;
            cx.nl(w)?;
            first = false;
        }
        cx.close_brace(w)?;
        Ok(())
    }
}

impl Emit for ClassBodyDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            ClassBodyDeclaration::Field(x) => x.emit(w, cx),
            ClassBodyDeclaration::Method(x) => x.emit(w, cx),
            ClassBodyDeclaration::Property(x) => x.emit(w, cx),
            ClassBodyDeclaration::Event(x) => x.emit(w, cx),
            ClassBodyDeclaration::Indexer(x) => x.emit(w, cx),
            ClassBodyDeclaration::Operator(x) => x.emit(w, cx),
            ClassBodyDeclaration::Constructor(x) => x.emit(w, cx),
            ClassBodyDeclaration::Destructor(x) => x.emit(w, cx),
            ClassBodyDeclaration::Record(x) => x.emit(w, cx),
            ClassBodyDeclaration::NestedClass(x) => x.emit(w, cx),
            ClassBodyDeclaration::NestedStruct(x) => x.emit(w, cx),
            ClassBodyDeclaration::NestedInterface(x) => x.emit(w, cx),
            ClassBodyDeclaration::NestedEnum(x) => x.emit(w, cx),
            ClassBodyDeclaration::NestedRecord(x) => x.emit(w, cx),
        }
    }
}
