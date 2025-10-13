use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::trivia::preprocessor::PreprocessorDirective;

impl Emit for PreprocessorDirective { fn emit<W: std::fmt::Write>(&self, _w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ todo!() } }
