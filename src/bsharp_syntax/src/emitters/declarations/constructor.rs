use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{ConstructorDeclaration, ConstructorInitializer};

impl Emit for ConstructorInitializer {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            ConstructorInitializer::Base(args) => {
                w.write_str(": base(")?;
                for (i, a) in args.iter().enumerate() { if i!=0 { w.write_str(", ")?; } a.emit(w, cx)?; }
                w.write_char(')')?;
            }
            ConstructorInitializer::This(args) => {
                w.write_str(": this(")?;
                for (i, a) in args.iter().enumerate() { if i!=0 { w.write_str(", ")?; } a.emit(w, cx)?; }
                w.write_char(')')?;
            }
        }
        Ok(())
    }
}
impl Emit for ConstructorDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Modifiers
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        // Name and parameters
        write!(w, "{}", self.name)?; w.write_char('(')?;
        for (i, p) in self.parameters.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; }
        w.write_char(')')?;
        // Initializer
        if let Some(init) = &self.initializer { w.write_char(' ')?; init.emit(w, cx)?; }
        // Body or semicolon (Allman style)
        if let Some(body) = &self.body {
            cx.nl(w)?; cx.write_indent(w)?; body.emit(w, cx)
        } else { w.write_char(';')?; Ok(()) }
    }
}
