use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{NullConditionalExpression, NullForgivingExpression};

impl Emit for NullConditionalExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.target.emit(w, cx)?;
        if self.is_element_access {
            w.write_str("?.[")?;
            if let Some(arg) = &self.argument {
                arg.emit(w, cx)?;
            }
            w.write_char(']')?;
        } else {
            w.write_str("?.")?;
            write!(w, "{}", self.member)?;
        }
        Ok(())
    }
}

impl Emit for NullForgivingExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.expr.emit(w, cx)?;
        w.write_char('!')?;
        Ok(())
    }
}
