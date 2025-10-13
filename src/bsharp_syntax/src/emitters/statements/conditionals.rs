use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::IfStatement;

impl Emit for IfStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
