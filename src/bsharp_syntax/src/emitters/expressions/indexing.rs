use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::ArrayIndexExpression;
use crate::expressions::indexing_expression::IndexingExpression;

impl Emit for IndexingExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx) ->Result<(),EmitError>{ todo!() } }
impl Emit for ArrayIndexExpression { fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
