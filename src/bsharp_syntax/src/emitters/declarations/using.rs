use crate::declarations::{GlobalUsingDirective, UsingDirective};
use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::statements::UsingStatement;

impl Emit for UsingDirective {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        match self {
            UsingDirective::Namespace { namespace } => {
                write!(w, "using {};", namespace)?;
            }
            UsingDirective::Alias {
                alias,
                namespace_or_type,
            } => {
                write!(w, "using {} = {};", alias, namespace_or_type)?;
            }
            UsingDirective::Static { type_name } => {
                write!(w, "using static {};", type_name)?;
            }
        }
        Ok(())
    }
}

impl Emit for UsingStatement {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        // 'await using' or 'using'
        if self.is_await {
            w.write_str("await ")?;
        }
        w.write_str("using")?;
        match &self.body {
            Some(body) => {
                // using-statement form: using (<resource-or-decl>) <body>
                w.write_str(" (")?;
                if let Some(decl) = &self.declaration {
                    decl.emit(w, cx)?;
                } else if let Some(expr) = &self.resource {
                    expr.emit(w, cx)?;
                }
                w.write_str(") ")?;
                body.emit(w, cx)
            }
            None => {
                // using-declaration form: using <declaration>;
                w.write_char(' ')?;
                if let Some(decl) = &self.declaration {
                    decl.emit(w, cx)?;
                    w.write_char(';')?;
                    Ok(())
                } else {
                    // Fallback: resource-only with semicolon (rare/non-standard), still emit
                    if let Some(expr) = &self.resource {
                        expr.emit(w, cx)?;
                    }
                    w.write_char(';')?;
                    Ok(())
                }
            }
        }
    }
}

impl Emit for GlobalUsingDirective {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        // C# syntax: global using <...>;
        w.write_str("global ")?;
        // Delegate to inner UsingDirective (which writes the trailing semicolon)
        self.using_directive.emit(w, cx)
    }
}
