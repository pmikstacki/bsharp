use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{LambdaExpression, LambdaParameter, LambdaParameterModifier, LambdaBody};

impl Emit for LambdaExpression {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        if self.is_async { w.write_str("async ")?; }
        // parameters
        if self.parameters.len() == 1 && self.parameters[0].ty.is_none() && self.parameters[0].modifier.is_none() {
            // single parameter shorthand
            write!(w, "{}", self.parameters[0].name)?;
        } else {
            w.write_char('(')?;
            for (i, p) in self.parameters.iter().enumerate() {
                if i != 0 { w.write_str(", ")?; }
                p.emit(w, cx)?;
            }
            w.write_char(')')?;
        }
        w.write_str(" => ")?;
        self.body.emit(w, cx)
    }
}

impl Emit for LambdaParameter {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{
        if let Some(m) = &self.modifier {
            match m { LambdaParameterModifier::Ref => w.write_str("ref ")?, LambdaParameterModifier::Out => w.write_str("out ")?, LambdaParameterModifier::In => w.write_str("in ")?, }
        }
        if let Some(ty) = &self.ty { write!(w, "{} ", ty)?; }
        write!(w, "{}", self.name)?;
        Ok(())
    }
}

impl Emit for LambdaParameterModifier {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{
        match self { LambdaParameterModifier::Ref => w.write_str("ref"), LambdaParameterModifier::Out => w.write_str("out"), LambdaParameterModifier::In => w.write_str("in"), }.map_err(EmitError::from)
    }
}

impl Emit for LambdaBody {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        match self {
            LambdaBody::ExpressionSyntax(e) => e.emit(w, cx),
            LambdaBody::Block(stmts) => {
                w.write_char('{')?; cx.nl(w)?; cx.push_indent();
                for s in stmts {
                    cx.write_indent(w)?; s.emit(w, cx)?; cx.nl(w)?;
                }
                cx.pop_indent(); cx.write_indent(w)?; w.write_char('}')?; Ok(())
            }
        }
    }
}
