use crate::declarations::LocalVariableDeclaration;
use crate::declarations::local_variable_declaration::VariableDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use std::fmt::Write;

impl Emit for VariableDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        write!(w, "{}", self.name)?;
        if let Some(init) = &self.initializer {
            write!(w, " = ")?;
            init.emit(w, cx)?;
        }
        Ok(())
    }
}

impl Emit for LocalVariableDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        if self.is_const {
            write!(w, "const ")?;
        }
        if self.is_ref {
            write!(w, "ref ")?;
        }
        self.declaration_type.emit(w, cx)?;
        write!(w, " ")?;
        for (i, d) in self.declarators.iter().enumerate() {
            if i != 0 {
                write!(w, ", ")?;
            }
            d.emit(w, cx)?;
        }
        Ok(())
    }
}
