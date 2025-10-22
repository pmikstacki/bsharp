use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{IndexExpression, RangeExpression};

impl Emit for RangeExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        if let Some(start) = &self.start {
            start.emit(w, cx)?;
        }
        w.write_str("..")?;
        if self.is_inclusive {
            w.write_char('=')?;
        }
        if let Some(end) = &self.end {
            end.emit(w, cx)?;
        }
        Ok(())
    }
}

impl Emit for IndexExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_char('^')?;
        self.value.emit(w, cx)
    }
}
