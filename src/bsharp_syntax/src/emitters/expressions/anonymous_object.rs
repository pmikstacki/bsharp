use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::AnonymousObjectCreationExpression;

impl Emit for AnonymousObjectCreationExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
