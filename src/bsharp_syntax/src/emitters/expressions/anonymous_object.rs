use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{AnonymousObjectCreationExpression, AnonymousObjectMember};

impl Emit for AnonymousObjectCreationExpression {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        w.write_str("new { ")?;
        for (i, m) in self.initializers.iter().enumerate() {
            if i != 0 { w.write_str(", ")?; }
            m.emit(w, cx)?;
        }
        w.write_str(" }")?;
        Ok(())
    }
}

impl Emit for AnonymousObjectMember {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        if let Some(name) = &self.name {
            write!(w, "{} = ", name)?;
            self.value.emit(w, cx)
        } else {
            // projection initializer
            self.value.emit(w, cx)
        }
    }
}
