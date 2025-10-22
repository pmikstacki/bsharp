use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::ArrayIndexExpression;
use crate::expressions::indexing_expression::IndexingExpression;

impl Emit for IndexingExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.target.emit(w, cx)?;
        w.write_char('[')?;
        self.index.emit(w, cx)?;
        w.write_char(']')?;
        Ok(())
    }
}

impl Emit for ArrayIndexExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.array.emit(w, cx)?;
        w.write_char('[')?;
        self.index.emit(w, cx)?;
        w.write_char(']')?;
        Ok(())
    }
}
