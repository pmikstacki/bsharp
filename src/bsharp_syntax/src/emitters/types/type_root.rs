use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::types::Type;

impl Emit for Type { fn emit<W: std::fmt::Write>(&self, _w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
