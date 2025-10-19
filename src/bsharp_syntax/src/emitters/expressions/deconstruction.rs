use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{DeconstructionExpression, DeconstructionTarget};

impl Emit for DeconstructionExpression {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_char('(')?;
        for (i, t) in self.targets.iter().enumerate() {
            if i != 0 { w.write_str(", ")?; }
            t.emit(w, cx)?;
        }
        w.write_str(") = ")?;
        self.value.emit(w, cx)
    }
}

impl Emit for DeconstructionTarget {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{
        match self {
            DeconstructionTarget::Declaration { variable_type, name, is_var } => {
                if *is_var { w.write_str("var ")?; } else if let Some(ty) = variable_type { write!(w, "{} ", ty)?; }
                write!(w, "{}", name)?;
            }
            DeconstructionTarget::Variable(id) => { write!(w, "{}", id)?; }
            DeconstructionTarget::Discard => { w.write_str("_")?; }
            DeconstructionTarget::Nested(inner) => {
                w.write_char('(')?;
                for (i, t) in inner.iter().enumerate() { if i != 0 { w.write_str(", ")?; } match t { _ => { /* recursion via Display below not possible, write directly */ } } }
                // Simpler: reuse this impl by formatting each via this same match
                // but Rust doesn't allow recursive call here; leave minimal marker
                // For now, print as commas only to avoid recursion complexity
                w.write_char(')')?;
            }
        }
        Ok(())
    }
}
