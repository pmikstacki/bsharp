use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::{BreakStatement, ContinueStatement, GotoStatement, LabelStatement, YieldStatement};

impl Emit for BreakStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{
        w.write_str("break;")?; Ok(())
    }
}

impl Emit for ContinueStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{
        w.write_str("continue;")?; Ok(())
    }
}

impl Emit for GotoStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{
        write!(w, "goto {};", self.label)?; Ok(())
    }
}

impl Emit for LabelStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W,_cx:&mut EmitCtx)->Result<(),EmitError>{
        write!(w, "{}:", self.label)?; Ok(())
    }
}

impl Emit for YieldStatement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        match self {
            YieldStatement::Return(e) => { w.write_str("yield return ")?; e.emit(w, cx)?; w.write_char(';')?; }
            YieldStatement::Break => { w.write_str("yield break;")?; }
        }
        Ok(())
    }
}
