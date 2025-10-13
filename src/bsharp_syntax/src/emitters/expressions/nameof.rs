use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::NameofExpression;

impl Emit for NameofExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
