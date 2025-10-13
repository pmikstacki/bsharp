use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{SwitchStatement, SwitchSection, SwitchLabel, GotoCaseStatement};

impl Emit for SwitchStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for SwitchSection { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for SwitchLabel { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for GotoCaseStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
