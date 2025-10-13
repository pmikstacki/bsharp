use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{NullConditionalExpression, NullForgivingExpression};

impl Emit for NullConditionalExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for NullForgivingExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
