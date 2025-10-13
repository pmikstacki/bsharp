use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::CheckedExpression;

impl Emit for CheckedExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
