use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::types::{Parameter, ParameterModifier};

impl Emit for Parameter {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // Attributes as [Attr] [Attr]
        for (i, a) in self.attributes.iter().enumerate() {
            if i != 0 {
                w.write_char(' ')?;
            }
            w.write_char('[')?;
            a.emit(w, cx)?;
            w.write_char(']')?;
        }
        if !self.attributes.is_empty() {
            w.write_char(' ')?;
        }
        if let Some(m) = &self.modifier {
            m.emit(w, cx)?;
            w.write_char(' ')?;
        }
        self.parameter_type.emit(w, cx)?;
        w.write_char(' ')?;
        write!(w, "{}", self.name)?;
        if let Some(def) = &self.default_value {
            w.write_str(" = ")?;
            def.emit(w, cx)?;
        }
        Ok(())
    }
}

impl Emit for ParameterModifier {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        let s = match self {
            ParameterModifier::Ref => "ref",
            ParameterModifier::Out => "out",
            ParameterModifier::In => "in",
            ParameterModifier::Params => "params",
            ParameterModifier::ScopedRef => "scoped ref",
            ParameterModifier::ScopedOut => "scoped out",
            ParameterModifier::ScopedIn => "scoped in",
        };
        w.write_str(s)?;
        Ok(())
    }
}
