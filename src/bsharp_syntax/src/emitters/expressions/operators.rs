use std::fmt::Write;
use crate::declarations::OperatorKind;
use crate::expressions::{AssignmentOperator, BinaryOperator, UnaryOperator};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for OperatorKind {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for AssignmentOperator {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for BinaryOperator {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}

impl Emit for UnaryOperator
{
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        todo!()
    }
}