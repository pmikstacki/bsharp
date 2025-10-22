use crate::declarations::MethodDeclaration;
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::statement::Statement;
use std::fmt::Write;

impl Emit for MethodDeclaration {
    fn emit<W: Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope(format!("Method({})", self.name));
        // Modifiers
        for (i, m) in self.modifiers.iter().enumerate() {
            if i != 0 {
                cx.space(w)?;
            }
            m.emit(w, cx)?;
        }
        if !self.modifiers.is_empty() {
            cx.space(w)?;
        }
        // Return type and name
        self.return_type.emit(w, cx)?;
        cx.space(w)?;
        write!(w, "{}", self.name)?;
        // Type parameters
        if let Some(tps) = &self.type_parameters {
            cx.token(w, "<")?;
            for (i, tp) in tps.iter().enumerate() {
                if i != 0 {
                    cx.token(w, ", ")?;
                }
                tp.emit(w, cx)?;
            }
            cx.token(w, ">")?;
        }
        // Parameters
        cx.token(w, "(")?;
        for (i, p) in self.parameters.iter().enumerate() {
            if i != 0 {
                cx.token(w, ", ")?;
            }
            p.emit(w, cx)?;
        }
        cx.token(w, ")")?;
        if let Some(cs) = &self.constraints {
            for c in cs {
                cx.space(w)?;
                c.emit(w, cx)?;
            }
        }
        // Header completed (method may or may not have a body)
        cx.trace_event(
            "header_done",
            &[
                ("has_body", self.body.is_some().to_string()),
                ("allman", "true".to_string()),
            ],
        );
        if let Some(body) = &self.body {
            match body {
                Statement::Block(stmts) => {
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
                    for (i, s) in stmts.iter().enumerate() {
                        cx.write_indent(w)?;
                        s.emit(w, cx)?;
                        cx.nl(w)?;
                        if i + 1 < stmts.len() {
                            let next = &stmts[i + 1];
                            cx.between_block_items(w, s, next)?;
                        }
                    }
                    cx.close_brace(w)
                }
                other => {
                    w.write_char(' ')?;
                    other.emit(w, cx)
                }
            }
        } else {
            w.write_char(';')?;
            Ok(())
        }
    }
}
