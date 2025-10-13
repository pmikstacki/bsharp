use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::types::CallingConvention;

impl Emit for CallingConvention { fn emit<W: std::fmt::Write>(&self, _w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
