use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{LambdaExpression, LambdaParameter, LambdaParameterModifier, LambdaBody};

impl Emit for LambdaExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for LambdaParameter { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for LambdaParameterModifier { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for LambdaBody { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
