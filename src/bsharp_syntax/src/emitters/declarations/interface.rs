use std::fmt::Write;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{InterfaceBodyDeclaration, InterfaceDeclaration};

impl Emit for InterfaceDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Attributes
        for al in &self.attributes { cx.write_indent(w)?; al.emit(w, cx)?; cx.nl(w)?; }
        // Modifiers and signature
        cx.write_indent(w)?;
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        w.write_str("interface ")?; write!(w, "{}", self.name)?;
        if let Some(tps) = &self.type_parameters { w.write_char('<')?; for (i,tp) in tps.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } tp.emit(w, cx)?; } w.write_char('>')?; }
        if !self.base_types.is_empty() { w.write_str(" : ")?; for (i, bt) in self.base_types.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } bt.emit(w, cx)?; } }
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

impl Emit for InterfaceBodyDeclaration{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            InterfaceBodyDeclaration::Method(x) => x.emit(w, cx),
            InterfaceBodyDeclaration::Property(x) => x.emit(w, cx),
            InterfaceBodyDeclaration::Event(x) => x.emit(w, cx),
            InterfaceBodyDeclaration::Indexer(x) => x.emit(w, cx),
            InterfaceBodyDeclaration::NestedClass(x) => x.emit(w, cx),
            InterfaceBodyDeclaration::NestedStruct(x) => x.emit(w, cx),
            InterfaceBodyDeclaration::NestedInterface(x) => x.emit(w, cx),
            InterfaceBodyDeclaration::NestedEnum(x) => x.emit(w, cx),
            InterfaceBodyDeclaration::NestedRecord(x) => x.emit(w, cx),
        }
    }

}