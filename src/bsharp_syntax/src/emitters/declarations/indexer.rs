use crate::declarations::{IndexerAccessor, IndexerAccessorList, IndexerDeclaration};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for IndexerAccessor {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Emit attributes and modifiers only; keyword and body handled by list emitter
        for (i, a) in self.attributes.iter().enumerate() {
            if i != 0 { w.write_char(' ')?; }
            // Attributes here are Attribute (not AttributeList); wrap in []
            w.write_char('[')?; a.emit(w, cx)?; w.write_char(']')?;
        }
        if !self.attributes.is_empty() { w.write_char(' ')?; }
        for (i, m) in self.modifiers.iter().enumerate() { if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        Ok(())
    }
}

impl Emit for IndexerDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Attributes (Attribute, wrap with [])
        for a in &self.attributes { cx.write_indent(w)?; w.write_char('[')?; a.emit(w, cx)?; w.write_char(']')?; cx.nl(w)?; }
        // Modifiers and signature
        cx.write_indent(w)?;
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        self.indexer_type.emit(w, cx)?; w.write_str(" this[")?;
        for (i, p) in self.parameters.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; }
        w.write_str("]")?;
        cx.nl(w)?; cx.write_indent(w)?;
        self.accessor_list.emit(w, cx)
    }
}

impl Emit for IndexerAccessorList {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        cx.open_brace(w)?;
        if let Some(get) = &self.get_accessor {
            cx.write_indent(w)?;
            get.emit(w, cx)?; // attrs + modifiers
            w.write_str("get")?;
            match &get.body { Some(b) => { w.write_char(' ')?; b.emit(w, cx)?; }, None => { w.write_char(';')?; } }
            cx.nl(w)?;
        }
        if let Some(set) = &self.set_accessor {
            cx.write_indent(w)?;
            set.emit(w, cx)?; // attrs + modifiers
            w.write_str("set")?;
            match &set.body { Some(b) => { w.write_char(' ')?; b.emit(w, cx)?; }, None => { w.write_char(';')?; } }
            cx.nl(w)?;
        }
        cx.close_brace(w)
    }
}