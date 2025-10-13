use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::types::{TypeParameter, Variance, TypeArgumentList};

impl Emit for TypeParameter { fn emit<W: std::fmt::Write>(&self, _w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for Variance { fn emit<W: std::fmt::Write>(&self, _w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for TypeArgumentList { fn emit<W: std::fmt::Write>(&self, _w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
