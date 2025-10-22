use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::declarations::{TypeParameterConstraint, TypeParameterConstraintClause};

impl Emit for TypeParameterConstraint {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            TypeParameterConstraint::ReferenceType => w.write_str("class")?,
            TypeParameterConstraint::ValueType => w.write_str("struct")?,
            TypeParameterConstraint::Unmanaged => w.write_str("unmanaged")?,
            TypeParameterConstraint::NotNull => w.write_str("notnull")?,
            TypeParameterConstraint::Constructor => w.write_str("new()")?,
            TypeParameterConstraint::SpecificType(t) => t.emit(w, cx)?,
            TypeParameterConstraint::SpecificParameter(id) => write!(w, "{}", id)?,
            TypeParameterConstraint::AllowsRefStruct => w.write_str("allows ref struct")?,
        }
        Ok(())
    }
}

impl Emit for TypeParameterConstraintClause {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        write!(w, "where {} : ", self.type_param)?;
        for (i, c) in self.constraints.iter().enumerate() {
            if i != 0 { w.write_str(", ")?; }
            c.emit(w, cx)?;
        }
        Ok(())
    }
}
