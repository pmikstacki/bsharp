use crate::declarations::OperatorKind;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{AssignmentOperator, BinaryOperator, UnaryOperator};
use std::fmt::Write;

impl Emit for OperatorKind {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        // OperatorKind is used in operator declarations, not general expressions.
        // Leave detailed formatting for declaration emitter milestone.
        let _ = (w, cx);
        Ok(())
    }
}

impl Emit for AssignmentOperator {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        let _ = cx;
        let s = match self {
            AssignmentOperator::Assign => "=",
            AssignmentOperator::AddAssign => "+=",
            AssignmentOperator::SubtractAssign => "-=",
            AssignmentOperator::MultiplyAssign => "*=",
            AssignmentOperator::DivideAssign => "/=",
            AssignmentOperator::ModuloAssign => "%=",
            AssignmentOperator::AndAssign => "&=",
            AssignmentOperator::OrAssign => "|=",
            AssignmentOperator::XorAssign => "^=",
            AssignmentOperator::LeftShiftAssign => "<<=",
            AssignmentOperator::RightShiftAssign => ">>=",
            AssignmentOperator::NullCoalescingAssign => "??=",
        };
        w.write_str(s)?;
        Ok(())
    }
}

impl Emit for BinaryOperator {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        let _ = cx;
        write!(w, "{}", self)?;
        Ok(())
    }
}

impl Emit for UnaryOperator {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        let _ = cx;
        write!(w, "{}", self)?;
        Ok(())
    }
}
