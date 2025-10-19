use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{SwitchStatement, SwitchSection, SwitchLabel, GotoCaseStatement};

impl Emit for SwitchStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("switch (")?;
        self.expression.emit(w, cx)?;
        w.write_str(")")?;
        cx.nl(w)?; cx.write_indent(w)?;
        cx.open_brace(w)?;
        for sec in &self.sections {
            cx.write_indent(w)?;
            sec.emit(w, cx)?;
            cx.nl(w)?;
        }
        cx.close_brace(w)
    }
}

impl Emit for SwitchSection {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        for (i, label) in self.labels.iter().enumerate() {
            if i != 0 { cx.nl(w)?; cx.write_indent(w)?; }
            label.emit(w, cx)?; w.write_char(':')?;
        }
        cx.nl(w)?;
        cx.push_indent();
        for stmt in &self.statements {
            cx.write_indent(w)?; stmt.emit(w, cx)?; cx.nl(w)?;
        }
        cx.pop_indent();
        Ok(())
    }
}

impl Emit for SwitchLabel {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        match self {
            SwitchLabel::Case(expr) => { w.write_str("case ")?; expr.emit(w, cx)?; }
            SwitchLabel::Default => { w.write_str("default")?; }
            SwitchLabel::Pattern { pattern, when_clause } => {
                w.write_str("case ")?; pattern.emit(w, cx)?;
                if let Some(when) = when_clause { w.write_str(" when ")?; when.emit(w, cx)?; }
            }
        }
        Ok(())
    }
}

impl Emit for GotoCaseStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        use crate::statements::goto_case_statement::GotoCaseKind;
        match &self.kind {
            GotoCaseKind::Case(expr) => { w.write_str("goto case ")?; expr.emit(w, cx)?; w.write_char(';')?; }
            GotoCaseKind::Default => { w.write_str("goto default;")?; }
        }
        Ok(())
    }
}
