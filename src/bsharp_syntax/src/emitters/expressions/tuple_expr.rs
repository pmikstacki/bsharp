use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{TupleExpression, TupleElement};

impl Emit for TupleExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for TupleElement { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
