use crate::declarations::{IndexerAccessor, IndexerAccessorList, IndexerDeclaration};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for IndexerAccessor {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Emit attributes and modifiers only; keyword and body handled by list emitter
        for (i, a) in self.attributes.iter().enumerate() {
            if i != 0 {
                cx.space(w)?;
            }
            // Attributes here are Attribute (not AttributeList); wrap in []
            cx.token(w, "[")?;
            a.emit(w, cx)?;
            cx.token(w, "]")?;
        }
        if !self.attributes.is_empty() {
            cx.space(w)?;
        }
        for (i, m) in self.modifiers.iter().enumerate() {
            if i != 0 {
                cx.space(w)?;
            }
            m.emit(w, cx)?;
        }
        if !self.modifiers.is_empty() {
            cx.space(w)?;
        }
        Ok(())
    }
}

impl Emit for IndexerDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope("Indexer".to_string());
        // Attributes (Attribute, wrap with [])
        for (i, a) in self.attributes.iter().enumerate() {
            if i != 0 {
                cx.write_indent(w)?;
            }
            cx.token(w, "[")?;
            a.emit(w, cx)?;
            cx.token(w, "]")?;
            cx.nl(w)?;
        }
        if !self.attributes.is_empty() {
            cx.write_indent(w)?;
        }
        // Modifiers and signature
        for (i, m) in self.modifiers.iter().enumerate() {
            if i != 0 {
                cx.space(w)?;
            }
            m.emit(w, cx)?;
        }
        if !self.modifiers.is_empty() {
            cx.space(w)?;
        }
        self.indexer_type.emit(w, cx)?;
        cx.token(w, " this[")?;
        for (i, p) in self.parameters.iter().enumerate() {
            if i != 0 {
                cx.token(w, ", ")?;
            }
            p.emit(w, cx)?;
        }
        cx.token(w, "]")?;
        cx.trace_event(
            "header_done",
            &[
                ("has_body", "true".to_string()),
                ("allman", "true".to_string()),
            ],
        );
        cx.nl(w)?;
        cx.write_indent(w)?;
        cx.trace_event(
            "before_open_brace",
            &[
                ("has_body", "true".to_string()),
                ("allman", "true".to_string()),
            ],
        );
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
            cx.token(w, "get")?;
            match &get.body {
                Some(b) => {
                    // Inline simple single-statement blocks
                    if let crate::statements::statement::Statement::Block(stmts) = b {
                        if stmts.len() == 1 {
                            cx.space(w)?;
                            w.write_str("{ ")?;
                            stmts[0].emit(w, cx)?;
                            w.write_str(" }")?;
                        } else {
                            cx.space(w)?;
                            b.emit(w, cx)?;
                        }
                    } else {
                        cx.space(w)?;
                        b.emit(w, cx)?;
                    }
                }
                None => {
                    cx.token(w, ";")?;
                }
            }
            cx.nl(w)?;
        }
        if let Some(set) = &self.set_accessor {
            cx.write_indent(w)?;
            set.emit(w, cx)?; // attrs + modifiers
            cx.token(w, "set")?;
            match &set.body {
                Some(b) => {
                    if let crate::statements::statement::Statement::Block(stmts) = b {
                        if stmts.len() == 1 {
                            cx.space(w)?;
                            w.write_str("{ ")?;
                            stmts[0].emit(w, cx)?;
                            w.write_str(" }")?;
                        } else {
                            cx.space(w)?;
                            b.emit(w, cx)?;
                        }
                    } else {
                        cx.space(w)?;
                        b.emit(w, cx)?;
                    }
                }
                None => {
                    cx.token(w, ";")?;
                }
            }
            cx.nl(w)?;
        }
        cx.close_brace(w)
    }
}
