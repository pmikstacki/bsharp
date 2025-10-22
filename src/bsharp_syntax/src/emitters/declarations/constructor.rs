use crate::declarations::{ConstructorDeclaration, ConstructorInitializer};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for ConstructorInitializer {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            ConstructorInitializer::Base(args) => {
                w.write_str(": base(")?;
                for (i, a) in args.iter().enumerate() {
                    if i != 0 {
                        w.write_str(", ")?;
                    }
                    a.emit(w, cx)?;
                }
                w.write_char(')')?;
            }
            ConstructorInitializer::This(args) => {
                w.write_str(": this(")?;
                for (i, a) in args.iter().enumerate() {
                    if i != 0 {
                        w.write_str(", ")?;
                    }
                    a.emit(w, cx)?;
                }
                w.write_char(')')?;
            }
        }
        Ok(())
    }
}
impl Emit for ConstructorDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope(format!("Constructor({})", self.name));
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
        // Name and parameters
        write!(w, "{}", self.name)?;
        cx.token(w, "(")?;
        for (i, p) in self.parameters.iter().enumerate() {
            if i != 0 {
                cx.token(w, ", ")?;
            }
            p.emit(w, cx)?;
        }
        cx.token(w, ")")?;
        // Initializer
        if let Some(init) = &self.initializer {
            cx.space(w)?;
            init.emit(w, cx)?;
        }
        // Header completed
        cx.trace_event(
            "header_done",
            &[
                ("has_body", self.body.is_some().to_string()),
                ("allman", "true".to_string()),
            ],
        );
        // Body or semicolon (Allman style for block)
        if let Some(body) = &self.body {
            match body {
                crate::statements::statement::Statement::Block(stmts) => {
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
                    for s in stmts {
                        cx.write_indent(w)?;
                        s.emit(w, cx)?;
                        cx.nl(w)?;
                    }
                    cx.close_brace(w)
                }
                other => {
                    cx.space(w)?;
                    other.emit(w, cx)
                }
            }
        } else {
            w.write_char(';')?;
            Ok(())
        }
    }
}
