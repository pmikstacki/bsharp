use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::InvocationExpression;

impl Emit for InvocationExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::{
            emitters::emit_trait::Emit as _, expressions::invocation_expression::ArgumentModifier,
        };
        // callee
        self.callee.emit(w, cx)?;
        // arguments
        w.write_char('(')?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i != 0 {
                w.write_str(", ")?;
            }
            if let Some(name) = &arg.name {
                write!(w, "{}: ", name)?;
            }
            if let Some(m) = &arg.modifier {
                match m {
                    ArgumentModifier::Ref => w.write_str("ref ")?,
                    ArgumentModifier::Out => w.write_str("out ")?,
                    ArgumentModifier::In => w.write_str("in ")?,
                }
            }
            arg.expr.emit(w, cx)?;
        }
        w.write_char(')')?;
        Ok(())
    }
}
