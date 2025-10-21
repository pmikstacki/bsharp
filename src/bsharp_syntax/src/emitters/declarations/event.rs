use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{EventAccessor, EventAccessorList, EventDeclaration};

impl Emit for EventAccessor {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        for al in &self.attributes { al.emit(w, cx)?; cx.space(w)?; }
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { cx.space(w)?; } m.emit(w, cx)?; }
        if self.body.is_some() { cx.space(w)?; }
        if let Some(b) = &self.body { b.emit(w, cx)?; } else { cx.token(w, ";")?; }
        Ok(())
    }
}
impl Emit for EventAccessorList {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        cx.open_brace(w)?;
        if let Some(add) = &self.add_accessor {
            cx.write_indent(w)?;
            for al in &add.attributes { al.emit(w, cx)?; cx.space(w)?; }
            for (i, m) in add.modifiers.iter().enumerate(){ if i!=0 { cx.space(w)?; } m.emit(w, cx)?; }
            if !add.modifiers.is_empty() || !add.attributes.is_empty() { cx.space(w)?; }
            cx.token(w, "add")?;
            match &add.body {
                Some(b) => {
                    // Special-case empty blocks to inline as { }
                    if let crate::statements::statement::Statement::Block(stmts) = b {
                        if stmts.is_empty() { cx.space(w)?; w.write_str("{ }")?; } else { cx.space(w)?; b.emit(w, cx)?; }
                    } else { cx.space(w)?; b.emit(w, cx)?; }
                }
                None => { cx.token(w, ";")?; }
            }
            cx.nl(w)?;
        }
        if let Some(remove) = &self.remove_accessor {
            cx.write_indent(w)?;
            for al in &remove.attributes { al.emit(w, cx)?; cx.space(w)?; }
            for (i, m) in remove.modifiers.iter().enumerate(){ if i!=0 { cx.space(w)?; } m.emit(w, cx)?; }
            if !remove.modifiers.is_empty() || !remove.attributes.is_empty() { cx.space(w)?; }
            cx.token(w, "remove")?;
            match &remove.body {
                Some(b) => {
                    if let crate::statements::statement::Statement::Block(stmts) = b {
                        if stmts.is_empty() { cx.space(w)?; w.write_str("{ }")?; } else { cx.space(w)?; b.emit(w, cx)?; }
                    } else { cx.space(w)?; b.emit(w, cx)?; }
                }
                None => { cx.token(w, ";")?; }
            }
            cx.nl(w)?;
        }
        cx.close_brace(w)
    }
}
impl Emit for EventDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope(format!("Event({})", self.name));
        for (i, al) in self.attributes.iter().enumerate() { if i != 0 { cx.write_indent(w)?; } al.emit(w, cx)?; cx.nl(w)?; }
        if !self.attributes.is_empty() { cx.write_indent(w)?; }
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { cx.space(w)?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { cx.space(w)?; }
        cx.token(w, "event ")?; self.event_type.emit(w, cx)?; cx.space(w)?; write!(w, "{}", self.name)?;
        if let Some(acclist) = &self.accessor_list {
            cx.trace_event("header_done", &[("has_body", "true".to_string()), ("allman", "true".to_string())]);
            cx.nl(w)?; cx.write_indent(w)?; cx.trace_event("before_open_brace", &[("has_body", "true".to_string()), ("allman", "true".to_string())]);
            acclist.emit(w, cx)
        } else {
            cx.trace_event("header_done", &[("has_body", "false".to_string()), ("allman", "false".to_string())]);
            cx.token(w, ";")?; Ok(())
        }
    }
}
