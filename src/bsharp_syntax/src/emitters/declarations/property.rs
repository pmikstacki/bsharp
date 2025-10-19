use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{PropertyAccessor, PropertyDeclaration};

impl Emit for PropertyAccessor {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            PropertyAccessor::Get { modifiers, attributes, body } => {
                for al in attributes { al.emit(w, cx)?; w.write_char(' ')?; }
                for (i, m) in modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
                if !modifiers.is_empty() { w.write_char(' ')?; }
                w.write_str("get")?;
                match body {
                    Some(stmt) => { w.write_char(' ')?; stmt.emit(w, cx)?; }
                    None => { w.write_char(';')?; }
                }
                Ok(())
            }
            PropertyAccessor::Set { modifiers, attributes, body } => {
                for al in attributes { al.emit(w, cx)?; w.write_char(' ')?; }
                for (i, m) in modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
                if !modifiers.is_empty() { w.write_char(' ')?; }
                w.write_str("set")?;
                match body {
                    Some(stmt) => { w.write_char(' ')?; stmt.emit(w, cx)?; }
                    None => { w.write_char(';')?; }
                }
                Ok(())
            }
            PropertyAccessor::Init { modifiers, attributes, body } => {
                for al in attributes { al.emit(w, cx)?; w.write_char(' ')?; }
                for (i, m) in modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
                if !modifiers.is_empty() { w.write_char(' ')?; }
                w.write_str("init")?;
                match body {
                    Some(stmt) => { w.write_char(' ')?; stmt.emit(w, cx)?; }
                    None => { w.write_char(';')?; }
                }
                Ok(())
            }
        }
    }
}

impl Emit for PropertyDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Attributes
        for al in &self.attributes { cx.write_indent(w)?; al.emit(w, cx)?; cx.nl(w)?; }
        // Modifiers + signature
        cx.write_indent(w)?;
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        self.property_type.emit(w, cx)?; w.write_char(' ')?; write!(w, "{}", self.name)?;
        let trivial = self.accessors.iter().all(|a| match a {
            PropertyAccessor::Get { modifiers, attributes, body }
            | PropertyAccessor::Set { modifiers, attributes, body }
            | PropertyAccessor::Init { modifiers, attributes, body } => {
                modifiers.is_empty() && attributes.is_empty() && body.is_none()
            }
        });

        if trivial {
            w.write_str(" { ")?;
            for (i, acc) in self.accessors.iter().enumerate() {
                if i != 0 { w.write_str(" ")?; }
                match acc {
                    PropertyAccessor::Get { .. } => w.write_str("get;")?,
                    PropertyAccessor::Set { .. } => w.write_str("set;")?,
                    PropertyAccessor::Init { .. } => w.write_str("init;")?,
                }
            }
            w.write_str(" }")?;
            if let Some(init) = &self.initializer { w.write_str(" = ")?; init.emit(w, cx)?; w.write_char(';')?; }
            Ok(())
        } else {
            cx.nl(w)?; cx.write_indent(w)?; cx.open_brace(w)?;
            for acc in &self.accessors { cx.write_indent(w)?; acc.emit(w, cx)?; cx.nl(w)?; }
            cx.close_brace(w)?;
            if let Some(init) = &self.initializer { w.write_str(" = ")?; init.emit(w, cx)?; w.write_char(';')?; }
            Ok(())
        }
    }
}
