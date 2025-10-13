use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{TryStatement, CatchClause, FinallyClause};

impl Emit for TryStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for CatchClause { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for FinallyClause { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
