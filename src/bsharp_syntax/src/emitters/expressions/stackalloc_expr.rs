use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::StackAllocExpression;

impl Emit for StackAllocExpression {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("stackalloc")?;
        // Type (optional for collection initializer form)
        if let Some(ty) = &self.target_type {
            w.write_char(' ')?;
            write!(w, "{}", ty)?;
        }
        // Count form: stackalloc T[count]
        if let Some(count) = &self.count {
            w.write_char('[')?;
            count.emit(w, cx)?;
            w.write_char(']')?;
            return Ok(());
        }
        // Initializer form: stackalloc T[] { items }
        if let Some(items) = &self.initializer {
            // If no explicit type provided, C# allows: stackalloc[] { ... }
            if self.target_type.is_some() { w.write_str("[]")?; } else { w.write_str("[]")?; }
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
