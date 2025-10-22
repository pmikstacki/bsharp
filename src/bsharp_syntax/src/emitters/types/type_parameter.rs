use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::types::{TypeArgumentList, TypeParameter, Variance};

impl Emit for TypeParameter {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        // variance (if any) followed by name
        self.variance.emit(w, cx)?;
        if !matches!(self.variance, Variance::None) {
            w.write_char(' ')?;
        }
        write!(w, "{}", self.name)?;
        Ok(())
    }
}

impl Emit for Variance {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        match self {
            Variance::None => Ok(()),
            Variance::In => w.write_str("in").map_err(EmitError::from),
            Variance::Out => w.write_str("out").map_err(EmitError::from),
        }
    }
}

impl Emit for TypeArgumentList {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_char('<')?;
        for (i, t) in self.types.iter().enumerate() {
            if i != 0 {
                w.write_str(", ")?;
            }
            t.emit(w, cx)?;
        }
        w.write_char('>')?;
        Ok(())
    }
}
