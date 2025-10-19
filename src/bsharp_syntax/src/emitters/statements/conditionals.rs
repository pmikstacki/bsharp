use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{IfStatement, statement::Statement};

impl Emit for IfStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("if (")?;
        self.condition.emit(w, cx)?;
        w.write_str(")")?;
        // Allman style: newline + indent before body
        cx.nl(w)?;
        cx.write_indent(w)?;
        self.consequence.emit(w, cx)?;
        if let Some(alt) = &self.alternative {
            match alt.as_ref() {
                // else if ... stays on a single line
                Statement::If(_) => {
                    cx.nl(w)?; cx.write_indent(w)?; w.write_str("else ")?; alt.emit(w, cx)?;
                }
                // else on its own line, brace on next line (Allman)
                _ => {
                    cx.nl(w)?; cx.write_indent(w)?; w.write_str("else")?;
                    cx.nl(w)?; cx.write_indent(w)?; alt.emit(w, cx)?;
                }
            }
        }
        Ok(())
    }
}
