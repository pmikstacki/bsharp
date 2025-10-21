use crate::declarations::OperatorDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use std::fmt::Write;

impl Emit for OperatorDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        use crate::declarations::operator_declaration::{OperatorKind, ConversionKind};
        let _scope = cx.node_scope("Operator".to_string());
        // Attributes (Attribute, wrap with [])
        for a in &self.attributes { cx.write_indent(w)?; cx.token(w, "[")?; a.emit(w, cx)?; cx.token(w, "]")?; cx.nl(w)?; }
        // Modifiers and signature
        for (i, m) in self.modifiers.iter().enumerate(){ if i!=0 { cx.space(w)?; } m.emit(w, cx)?; }
        if !self.modifiers.is_empty() { cx.space(w)?; }
        match &self.operator {
            OperatorKind::Unary(op) => { self.return_type.emit(w, cx)?; cx.token(w, " operator ")?; write!(w, "{}", op)?; }
            OperatorKind::Binary(op) => { self.return_type.emit(w, cx)?; cx.token(w, " operator ")?; write!(w, "{}", op)?; }
            OperatorKind::Conversion { kind, target_type } => {
                match kind { ConversionKind::Implicit => cx.token(w, "implicit")?, ConversionKind::Explicit => cx.token(w, "explicit")?, };
                cx.token(w, " operator ")?; target_type.emit(w, cx)?;
            }
        }
        cx.token(w, "(")?;
        // Parameters
        for (i, p) in self.parameters.iter().enumerate(){ if i!=0 { cx.token(w, ", ")?; } p.emit(w, cx)?; }
        cx.token(w, ")")?;
        // Body string (already includes braces/semicolon per AST design)
        cx.space(w)?; cx.token(w, &self.body)?;
        Ok(())
    }
}
