use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::AwaitExpression;

impl Emit for AwaitExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
