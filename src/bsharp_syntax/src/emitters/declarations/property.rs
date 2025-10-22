use crate::declarations::{PropertyAccessor, PropertyDeclaration};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};

impl Emit for PropertyAccessor {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            PropertyAccessor::Get {
                modifiers,
                attributes,
                body,
            } => {
                for al in attributes {
                    al.emit(w, cx)?;
                    cx.space(w)?;
                }
                for (i, m) in modifiers.iter().enumerate() {
                    if i != 0 {
                        cx.space(w)?;
                    }
                    m.emit(w, cx)?;
                }
                if !modifiers.is_empty() {
                    cx.space(w)?;
                }
                cx.token(w, "get")?;
                match body {
                    Some(stmt) => {
                        cx.space(w)?;
                        if let crate::statements::statement::Statement::Block(stmts) = stmt {
                            if stmts.is_empty() {
                                w.write_str("{ }")?;
                            } else if stmts.len() == 1 {
                                w.write_str("{ ")?;
                                stmts[0].emit(w, cx)?;
                                w.write_str(" }")?;
                            } else {
                                stmt.emit(w, cx)?;
                            }
                        } else {
                            stmt.emit(w, cx)?;
                        }
                    }
                    None => {
                        cx.token(w, ";")?;
                    }
                }
                Ok(())
            }
            PropertyAccessor::Set {
                modifiers,
                attributes,
                body,
            } => {
                for al in attributes {
                    al.emit(w, cx)?;
                    cx.space(w)?;
                }
                for (i, m) in modifiers.iter().enumerate() {
                    if i != 0 {
                        cx.space(w)?;
                    }
                    m.emit(w, cx)?;
                }
                if !modifiers.is_empty() {
                    cx.space(w)?;
                }
                cx.token(w, "set")?;
                match body {
                    Some(stmt) => {
                        cx.space(w)?;
                        if let crate::statements::statement::Statement::Block(stmts) = stmt {
                            if stmts.is_empty() {
                                w.write_str("{ }")?;
                            } else if stmts.len() == 1 {
                                w.write_str("{ ")?;
                                stmts[0].emit(w, cx)?;
                                w.write_str(" }")?;
                            } else {
                                stmt.emit(w, cx)?;
                            }
                        } else {
                            stmt.emit(w, cx)?;
                        }
                    }
                    None => {
                        cx.token(w, ";")?;
                    }
                }
                Ok(())
            }
            PropertyAccessor::Init {
                modifiers,
                attributes,
                body,
            } => {
                for al in attributes {
                    al.emit(w, cx)?;
                    cx.space(w)?;
                }
                for (i, m) in modifiers.iter().enumerate() {
                    if i != 0 {
                        cx.space(w)?;
                    }
                    m.emit(w, cx)?;
                }
                if !modifiers.is_empty() {
                    cx.space(w)?;
                }
                cx.token(w, "init")?;
                match body {
                    Some(stmt) => {
                        cx.space(w)?;
                        if let crate::statements::statement::Statement::Block(stmts) = stmt {
                            if stmts.is_empty() {
                                w.write_str("{ }")?;
                            } else if stmts.len() == 1 {
                                w.write_str("{ ")?;
                                stmts[0].emit(w, cx)?;
                                w.write_str(" }")?;
                            } else {
                                stmt.emit(w, cx)?;
                            }
                        } else {
                            stmt.emit(w, cx)?;
                        }
                    }
                    None => {
                        cx.token(w, ";")?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl Emit for PropertyDeclaration {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope(format!("Property({})", self.name));
        // Attributes
        for (i, al) in self.attributes.iter().enumerate() {
            if i != 0 {
                cx.write_indent(w)?;
            }
            al.emit(w, cx)?;
            cx.nl(w)?;
        }
        if !self.attributes.is_empty() {
            cx.write_indent(w)?;
        }
        // Modifiers + signature
        for (i, m) in self.modifiers.iter().enumerate() {
            if i != 0 {
                cx.space(w)?;
            }
            m.emit(w, cx)?;
        }
        if !self.modifiers.is_empty() {
            cx.space(w)?;
        }
        self.property_type.emit(w, cx)?;
        cx.space(w)?;
        write!(w, "{}", self.name)?;
        let trivial = self.accessors.iter().all(|a| match a {
            PropertyAccessor::Get {
                modifiers,
                attributes,
                body,
            }
            | PropertyAccessor::Set {
                modifiers,
                attributes,
                body,
            }
            | PropertyAccessor::Init {
                modifiers,
                attributes,
                body,
            } => modifiers.is_empty() && attributes.is_empty() && body.is_none(),
        });

        if trivial {
            // Header done; trivial accessor body will be inline, not Allman
            cx.trace_event(
                "header_done",
                &[
                    ("has_body", "true".to_string()),
                    ("allman", "false".to_string()),
                ],
            );
            cx.token(w, " { ")?;
            for (i, acc) in self.accessors.iter().enumerate() {
                if i != 0 {
                    cx.space(w)?;
                }
                match acc {
                    PropertyAccessor::Get { .. } => cx.token(w, "get;")?,
                    PropertyAccessor::Set { .. } => cx.token(w, "set;")?,
                    PropertyAccessor::Init { .. } => cx.token(w, "init;")?,
                }
            }
            cx.token(w, " }")?;
            if let Some(init) = &self.initializer {
                cx.token(w, " = ")?;
                init.emit(w, cx)?;
                cx.token(w, ";")?;
            }
            Ok(())
        } else {
            // Header done; non-trivial accessor body with Allman braces
            cx.trace_event(
                "header_done",
                &[
                    ("has_body", "true".to_string()),
                    ("allman", "true".to_string()),
                ],
            );
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
            for acc in &self.accessors {
                cx.write_indent(w)?;
                acc.emit(w, cx)?;
                cx.nl(w)?;
            }
            cx.close_brace(w)?;
            if let Some(init) = &self.initializer {
                cx.token(w, " = ")?;
                init.emit(w, cx)?;
                cx.token(w, ";")?;
            }
            Ok(())
        }
    }
}
