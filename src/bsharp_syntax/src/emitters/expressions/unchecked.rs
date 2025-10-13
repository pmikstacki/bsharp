use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::UncheckedExpression;

impl Emit for UncheckedExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
