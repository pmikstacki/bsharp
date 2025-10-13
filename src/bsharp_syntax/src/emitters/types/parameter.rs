use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::types::{Parameter, ParameterModifier};

impl Emit for Parameter { fn emit<W: std::fmt::Write>(&self, _w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for ParameterModifier { fn emit<W: std::fmt::Write>(&self, _w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
