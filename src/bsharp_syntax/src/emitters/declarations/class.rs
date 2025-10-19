use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{ClassBodyDeclaration, ClassDeclaration};

impl Emit for ClassDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Attributes
        for al in &self.attributes {
            cx.write_indent(w)?; al.emit(w, cx)?; cx.nl(w)?;
        }
        // Documentation (skip for now)
        // Modifiers
        cx.write_indent(w)?;
        for (i, m) in self.modifiers.iter().enumerate() { if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        // Keyword and name
        w.write_str("class ")?; write!(w, "{}", self.name)?;
        // Type parameters
        if let Some(tps) = &self.type_parameters { w.write_char('<')?; for (i,tp) in tps.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } tp.emit(w, cx)?; } w.write_char('>')?; }
        // Primary constructor parameters (C# 12)
        if let Some(params) = &self.primary_constructor_parameters { w.write_char('(')?; for (i,p) in params.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; } w.write_char(')')?; }
        // Base types
        if !self.base_types.is_empty() {
            w.write_str(" : ")?;
            for (i, bt) in self.base_types.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } bt.emit(w, cx)?; }
        }
        if let Some(cs) = &self.constraints { for c in cs { w.write_char(' ')?; c.emit(w, cx)?; } }
        cx.nl(w)?;
        cx.write_indent(w)?;
        cx.open_brace(w)?;
        let mut first = true;
        for d in &self.body_declarations {
            if !first { cx.between_members(w)?; }
            cx.write_indent(w)?; d.emit(w, cx)?; cx.nl(w)?;
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
