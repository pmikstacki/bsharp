use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{CheckedStatement, UncheckedStatement, LockStatement, FixedStatement, UnsafeStatement, LocalFunctionStatement, ForInitializer};

impl Emit for CheckedStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for UncheckedStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for LockStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for FixedStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for UnsafeStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for LocalFunctionStatement { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
impl Emit for ForInitializer { fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
