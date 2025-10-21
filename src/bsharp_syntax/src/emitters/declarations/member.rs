use std::fmt::Write;
use crate::declarations::{MemberBody, MemberDeclaration};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::statement::Statement;

impl Emit for MemberBody{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            MemberBody::Block(stmt) => stmt.emit(w, cx),
            MemberBody::Expression(expr) => { w.write_str("=> ")?; expr.emit(w, cx)?; w.write_char(';')?; Ok(()) }
            MemberBody::None => { w.write_char(';')?; Ok(()) }
        }
    }
}

impl Emit for MemberDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Modifiers
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }

        let is_ctor = self.return_type.is_none();
        if let Some(ret) = &self.return_type { ret.emit(w, cx)?; w.write_char(' ')?; }
        write!(w, "{}", self.name)?;

        // Type parameters
        if let Some(tps) = &self.type_parameters { w.write_char('<')?; for (i,tp) in tps.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } tp.emit(w, cx)?; } w.write_char('>')?; }
        // Parameters
        w.write_char('(')?; for (i,p) in self.parameters.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; } w.write_char(')')?;
        // Initializer (only for constructors)
        if is_ctor { if let Some(init) = &self.initializer { w.write_char(' ')?; init.emit(w, cx)?; } }
        // Constraints
        if let Some(cs) = &self.constraints { for c in cs { w.write_char(' ')?; c.emit(w, cx)?; } }
        // Body or semicolon (Allman for block bodies)
        if let Some(body) = &self.body {
            match body {
                Statement::Block(_) => { cx.nl(w)?; cx.write_indent(w)?; body.emit(w, cx) }
                _ => { w.write_char(' ')?; body.emit(w, cx) }
            }
        } else { w.write_char(';')?; Ok(()) }
    }
}

// MemberAccessExpression Emit is implemented in expressions emitter module.
