use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{DeconstructionExpression, DeconstructionTarget};

impl Emit for DeconstructionExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for DeconstructionTarget { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
