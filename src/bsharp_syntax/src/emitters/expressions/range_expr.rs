use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{RangeExpression, IndexExpression};

impl Emit for RangeExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for IndexExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
