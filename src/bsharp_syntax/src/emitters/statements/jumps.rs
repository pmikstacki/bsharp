use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{BreakStatement, ContinueStatement, GotoStatement, LabelStatement, YieldStatement};

impl Emit for BreakStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for ContinueStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for GotoStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for LabelStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for YieldStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
