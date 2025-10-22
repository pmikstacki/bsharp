use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::statement::Statement;

impl Emit for Statement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        match self {
            Statement::Goto(s) => s.emit(w, cx),
            Statement::GotoCase(s) => s.emit(w, cx),
            Statement::Label(s) => s.emit(w, cx),
            Statement::Checked(b) => b.emit(w, cx),
            Statement::Unchecked(b) => b.emit(w, cx),
            Statement::Lock(b) => b.emit(w, cx),
            Statement::Using(b) => b.emit(w, cx),
            Statement::Yield(y) => y.emit(w, cx),
            Statement::Unsafe(b) => b.emit(w, cx),
            Statement::Fixed(b) => b.emit(w, cx),
            Statement::Try(b) => b.emit(w, cx),
            Statement::ForEach(b) => b.emit(w, cx),
            Statement::Switch(b) => b.emit(w, cx),
            Statement::DoWhile(b) => b.emit(w, cx),
            Statement::Break(b) => b.emit(w, cx),
            Statement::Continue(c) => c.emit(w, cx),
            Statement::For(b)  => b.emit(w, cx),
            Statement::While(b)=> b.emit(w, cx),
            Statement::If(b)   => b.emit(w, cx),
            Statement::Declaration(d) => { d.emit(w, cx)?; w.write_char(';') }
                .map_err(EmitError::from),
            Statement::LocalFunction(f) => f.emit(w, cx),
            Statement::Expression(e) => { e.emit(w, cx)?; w.write_char(';')?; Ok(()) }
            Statement::Return(opt) => {
                w.write_str("return")?;
                if let Some(e) = opt { w.write_char(' ')?; e.emit(w, cx)?; }
                w.write_char(';')?; Ok(())
            }
            Statement::Throw(opt) => {
                w.write_str("throw")?;
                if let Some(e) = opt { w.write_char(' ')?; e.emit(w, cx)?; }
                w.write_char(';')?; Ok(())
            }
            Statement::Block(stmts) => {
                if stmts.is_empty() { w.write_str("{ }")?; return Ok(()); }
                w.write_char('{')?; cx.nl(w)?; cx.push_indent();
                for (i, s) in stmts.iter().enumerate() {
                    cx.write_indent(w)?;
                    s.emit(w, cx)?;
                    cx.nl(w)?;
                    if i + 1 < stmts.len() {
                        let next = &stmts[i + 1];
                        cx.between_block_items(w, s, next)?;
                    }
                }
                cx.pop_indent(); cx.write_indent(w)?; w.write_char('}')?; Ok(())
            }
            Statement::Empty => { w.write_char(';')?; Ok(()) }
            Statement::Deconstruction(d) => { d.emit(w, cx)?; w.write_char(';')?; Ok(()) }
        }
    }
}
