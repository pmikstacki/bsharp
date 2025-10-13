use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{DoWhileStatement, WhileStatement, ForStatement, ForEachStatement};

impl Emit for DoWhileStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for WhileStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for ForStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for ForEachStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
