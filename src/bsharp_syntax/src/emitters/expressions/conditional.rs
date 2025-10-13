use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::ConditionalExpression;

impl Emit for ConditionalExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
