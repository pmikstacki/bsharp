use crate::declarations::OperatorDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use std::fmt::Write;

impl Emit for OperatorDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        use crate::declarations::operator_declaration::{OperatorKind, ConversionKind};
        // Attributes (Attribute, wrap with [])
        for a in &self.attributes { cx.write_indent(w)?; w.write_char('[')?; a.emit(w, cx)?; w.write_char(']')?; cx.nl(w)?; }
        // Modifiers and signature
        cx.write_indent(w)?;
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { w.write_char(' ')?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { w.write_char(' ')?; }
        self.return_type.emit(w, cx)?; w.write_str(" operator ")?;
        match &self.operator {
            OperatorKind::Unary(op) => { write!(w, "{}", op)?; }
            OperatorKind::Binary(op) => { write!(w, "{}", op)?; }
            OperatorKind::Conversion { kind, target_type } => {
                match kind { ConversionKind::Implicit => w.write_str("implicit")?, ConversionKind::Explicit => w.write_str("explicit")?, };
                w.write_str(" operator ")?; target_type.emit(w, cx)?;
            }
        }
        w.write_char('(')?;
        // Parameters
        for (i, p) in self.parameters.iter().enumerate(){ if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; }
        w.write_char(')')?;
        // Body string (already includes braces/semicolon per AST design)
        w.write_char(' ')?; w.write_str(&self.body)?;
        Ok(())
    }
}
