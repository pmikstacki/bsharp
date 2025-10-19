use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{EventAccessor, EventAccessorList, EventDeclaration};

impl Emit for EventAccessor {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        for al in &self.attributes { al.emit(w, cx)?; w.write_char(' ')?; }
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if self.body.is_some() { w.write_char(' ')?; }
        if let Some(b) = &self.body { b.emit(w, cx)?; } else { w.write_char(';')?; }
        Ok(())
    }
}
impl Emit for EventAccessorList {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        cx.open_brace(w)?;
        if let Some(add) = &self.add_accessor {
            cx.write_indent(w)?;
            for al in &add.attributes { al.emit(w, cx)?; w.write_char(' ')?; }
            for (i, m) in add.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
            if !add.modifiers.is_empty() || !add.attributes.is_empty() { w.write_char(' ')?; }
            w.write_str("add")?;
            match &add.body { Some(b) => { w.write_char(' ')?; b.emit(w, cx)?; }, None => { w.write_char(';')?; } }
            cx.nl(w)?;
        }
        if let Some(remove) = &self.remove_accessor {
            cx.write_indent(w)?;
            for al in &remove.attributes { al.emit(w, cx)?; w.write_char(' ')?; }
            for (i, m) in remove.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
            if !remove.modifiers.is_empty() || !remove.attributes.is_empty() { w.write_char(' ')?; }
            w.write_str("remove")?;
            match &remove.body { Some(b) => { w.write_char(' ')?; b.emit(w, cx)?; }, None => { w.write_char(';')?; } }
            cx.nl(w)?;
        }
        cx.close_brace(w)
    }
}
impl Emit for EventDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        for al in &self.attributes { cx.write_indent(w)?; al.emit(w, cx)?; cx.nl(w)?; }
        cx.write_indent(w)?;
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        w.write_str("event ")?; self.event_type.emit(w, cx)?; w.write_char(' ')?; write!(w, "{}", self.name)?;
        if let Some(acclist) = &self.accessor_list { cx.nl(w)?; cx.write_indent(w)?; acclist.emit(w, cx) } else { w.write_char(';')?; Ok(()) }
    }
}
