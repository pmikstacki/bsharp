use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::NewExpression;

impl Emit for NewExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("new")?;
        if let Some(ty) = &self.target_type {
            w.write_char(' ')?;
            write!(w, "{}", ty)?;
        }
        // Arguments
        w.write_char('(')?;
        for (i, a) in self.arguments.iter().enumerate() {
            if i != 0 { w.write_str(", ")?; }
            a.emit(w, cx)?;
        }
        w.write_char(')')?;

        // Object initializer
        if let Some(inits) = &self.object_initializer {
            w.write_str(" { ")?;
            for (i, init) in inits.iter().enumerate() {
                if i != 0 { w.write_str(", ")?; }
                init.emit(w, cx)?;
            }
            w.write_str(" }")?;
        }
        // Collection initializer
        if let Some(items) = &self.collection_initializer {
            w.write_str(" { ")?;
            for (i, it) in items.iter().enumerate() {
                if i != 0 { w.write_str(", ")?; }
                it.emit(w, cx)?;
            }
            w.write_str(" }")?;
        }
        Ok(())
    }
}
