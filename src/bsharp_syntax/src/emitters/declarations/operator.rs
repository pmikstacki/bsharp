use crate::declarations::OperatorDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use std::fmt::Write;

impl Emit for OperatorDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::declarations::operator_declaration::{ConversionKind, OperatorKind};
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope("Operator".to_string());
        // Attributes (Attribute, wrap with [])
        for a in &self.attributes {
            cx.write_indent(w)?;
            cx.token(w, "[")?;
            a.emit(w, cx)?;
            cx.token(w, "]")?;
            cx.nl(w)?;
        }
        // Modifiers and signature
        for (i, m) in self.modifiers.iter().enumerate() {
            if i != 0 {
                cx.space(w)?;
            }
            m.emit(w, cx)?;
        }
        if !self.modifiers.is_empty() {
            cx.space(w)?;
        }
        match &self.operator {
            OperatorKind::Unary(op) => {
                self.return_type.emit(w, cx)?;
                cx.token(w, " operator ")?;
                write!(w, "{}", op)?;
            }
            OperatorKind::Binary(op) => {
                self.return_type.emit(w, cx)?;
                cx.token(w, " operator ")?;
                write!(w, "{}", op)?;
            }
            OperatorKind::Conversion { kind, target_type } => {
                match kind {
                    ConversionKind::Implicit => cx.token(w, "implicit")?,
                    ConversionKind::Explicit => cx.token(w, "explicit")?,
                };
                cx.token(w, " operator ")?;
                target_type.emit(w, cx)?;
            }
        }
        cx.token(w, "(")?;
        // Parameters
        for (i, p) in self.parameters.iter().enumerate() {
            if i != 0 {
                cx.token(w, ", ")?;
            }
            p.emit(w, cx)?;
        }
        cx.token(w, ")")?;
        // Body handling
        let body_trimmed = self.body.trim();
        if body_trimmed.starts_with('{') {
            // Allman: newline + indent + open brace; indent each inner line; close brace
            cx.trace_event(
                "header_done",
                &[
                    ("has_body", "true".to_string()),
                    ("allman", "true".to_string()),
                ],
            );
            // Extract inner region between first '{' and last '}'
            let s = body_trimmed;
            let start_idx = s.find('{').unwrap_or(0) + 1;
            let end_idx = if s.ends_with('}') && s.len() >= 2 {
                s.len() - 1
            } else {
                s.len()
            };
            let mut inner = &s[start_idx..end_idx];
            if inner.starts_with('\n') {
                inner = &inner[1..];
            }
            cx.nl(w)?;
            cx.write_indent(w)?;
            cx.trace_event(
                "before_open_brace",
                &[
                    ("has_body", "true".to_string()),
                    ("allman", "true".to_string()),
                ],
            );
            cx.open_brace(w)?;
            if !inner.is_empty() {
                for (_i, line) in inner.split('\n').enumerate() {
                    cx.write_indent(w)?;
                    cx.token(w, line)?;
                    cx.nl(w)?;
                }
            }
            cx.close_brace(w)
        } else {
            // Expression-bodied or semicolon form stays inline
            cx.trace_event(
                "header_done",
                &[
                    ("has_body", "false".to_string()),
                    ("allman", "false".to_string()),
                ],
            );
            cx.space(w)?;
            cx.token(w, &self.body)?;
            Ok(())
        }
    }
}
