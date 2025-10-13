use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{TypeParameterConstraint, TypeParameterConstraintClause};

impl Emit for TypeParameterConstraint {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}

impl Emit for TypeParameterConstraintClause {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> { todo!() }
}
