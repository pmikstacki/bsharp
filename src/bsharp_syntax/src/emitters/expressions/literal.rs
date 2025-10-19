use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::Literal;

impl Emit for Literal {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        match self {
            Literal::Integer(v) => write!(w, "{}", v)?,
            Literal::IntegerWithSuffix(v, sfx) => {
                use crate::expressions::literal::IntegerSuffix::*;
                match sfx {
                    U => write!(w, "{}u", v)?,
                    L => write!(w, "{}L", v)?,
                    UL => write!(w, "{}UL", v)?,
                }
            }
            Literal::Float(v) => write!(w, "{}", v)?,
            Literal::Decimal(txt) => write!(w, "{}", txt)?,
            Literal::Boolean(b) => w.write_str(if *b { "true" } else { "false" })?,
            Literal::String(s) => {
                w.write_char('"')?;
                for ch in s.chars() {
                    match ch {
                        '"' => w.write_str("\\\"")?,
                        '\\' => w.write_str("\\\\")?,
                        '\n' => w.write_str("\\n")?,
                        '\r' => w.write_str("\\r")?,
                        '\t' => w.write_str("\\t")?,
                        c => w.write_char(c)?,
                    }
                }
                w.write_char('"')?;
            }
            Literal::Utf8String(bytes) => {
                // Best-effort: print as ASCII with escapes, then append u8 suffix
                w.write_char('"')?;
                for &b in bytes {
                    let c = b as char;
                    if c.is_ascii_graphic() || c == ' ' {
                        if c == '"' { w.write_str("\\\"")?; }
                        else if c == '\\' { w.write_str("\\\\")?; }
                        else { w.write_char(c)?; }
                    } else {
                        write!(w, "\\x{:02X}", b)?;
                    }
                }
                w.write_char('"')?;
                w.write_str("u8")?;
            }
            Literal::Char(c) => {
                w.write_char('\'')?; // opening quote
                match c {
                    '\'' => w.write_str("\\'")?,
                    '\\' => w.write_str("\\\\")?,
                    '\n' => w.write_str("\\n")?,
                    '\r' => w.write_str("\\r")?,
                    '\t' => w.write_str("\\t")?,
                    ch => w.write_char(*ch)?,
                }
                w.write_char('\'')?; // closing quote
            }
            Literal::Null => w.write_str("null")?,
            Literal::InterpolatedString(is) => {
                // Basic interpolation without verbatim/raw variants for now
                w.write_char('$')?;
                w.write_char('"')?;
                for part in &is.parts {
                    use crate::expressions::literal::InterpolatedStringPart as P;
                    match part {
                        P::Text(t) => {
                            for ch in t.chars() {
                                match ch {
                                    '"' => w.write_str("\\\"")?,
                                    '\\' => w.write_str("\\\\")?,
                                    '\n' => w.write_str("\\n")?,
                                    '\r' => w.write_str("\\r")?,
                                    '\t' => w.write_str("\\t")?,
                                    c => w.write_char(c)?,
                                }
                            }
                        }
                        P::Interpolation { expression, alignment, format_string } => {
                            w.write_char('{')?;
                            use crate::emitters::emit_trait::Emit as _;
                            expression.emit(w, &mut EmitCtx::new())?;
                            if let Some(align) = alignment {
                                w.write_char(',')?;
                                align.emit(w, &mut EmitCtx::new())?;
                            }
                            if let Some(fmt) = format_string {
                                w.write_char(':')?;
                                w.write_str(fmt)?;
                            }
                            w.write_char('}')?;
                        }
                    }
                }
                w.write_char('"')?;
            }
            Literal::VerbatimString(s) => {
                w.write_str("@\"")?;
                for ch in s.chars() {
                    match ch {
                        '"' => w.write_str("\"\"")?, // doubled quotes in verbatim
                        c => w.write_char(c)?,
                    }
                }
                w.write_char('"')?;
            }
            Literal::RawString(s) => {
                // Assume s already contains proper raw content including quotes.
                w.write_str(s)?;
            }
        }
        Ok(())
    }
}
