use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::statement::Statement;

impl Emit for Statement { fn emit<W: std::fmt::Write>(&self, _w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
