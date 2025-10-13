use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::ast::CompilationUnit;

impl Emit for CompilationUnit { fn emit<W: std::fmt::Write>(&self, _w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
