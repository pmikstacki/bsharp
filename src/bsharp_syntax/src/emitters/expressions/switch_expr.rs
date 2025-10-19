use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::expression::{SwitchExpression, SwitchExpressionArm};

impl Emit for SwitchExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.expression.emit(w, cx)?;
        cx.space(w)?; w.write_str("switch")?; cx.space(w)?;
        w.write_char('{')?;
        // single-line arms separated by comma and space for now
        if !self.arms.is_empty() { cx.space(w)?; }
        for (i, arm) in self.arms.iter().enumerate() {
            if i != 0 { w.write_str(", ")?; }
            arm.emit(w, cx)?;
        }
        if !self.arms.is_empty() { cx.space(w)?; }
        w.write_char('}')?;
        Ok(())
    }
}

impl Emit for SwitchExpressionArm {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.pattern.emit(w, cx)?;
        if let Some(when) = &self.when_clause {
            cx.space(w)?; w.write_str("when")?; cx.space(w)?; when.emit(w, cx)?;
        }
        cx.space(w)?; w.write_str("=>")?; cx.space(w)?;
        self.expression.emit(w, cx)
    }
}
