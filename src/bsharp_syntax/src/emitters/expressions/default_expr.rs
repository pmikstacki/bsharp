use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::DefaultExpression;

impl Emit for DefaultExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        match &self.target_type {
            Some(t) => write!(w, "default({})", t)?,
            None => w.write_str("default")?,
        }
        Ok(())
    }
}
