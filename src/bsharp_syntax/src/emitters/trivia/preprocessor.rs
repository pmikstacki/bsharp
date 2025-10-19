use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::trivia::preprocessor::PreprocessorDirective;

impl Emit for PreprocessorDirective {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{
        match self {
            PreprocessorDirective::Define { symbol } => write!(w, "#define {}", symbol)?,
            PreprocessorDirective::Undef { symbol } => write!(w, "#undef {}", symbol)?,
            PreprocessorDirective::If { condition } => write!(w, "#if {}", condition)?,
            PreprocessorDirective::Elif { condition } => write!(w, "#elif {}", condition)?,
            PreprocessorDirective::Else => w.write_str("#else")?,
            PreprocessorDirective::Endif => w.write_str("#endif")?,
            PreprocessorDirective::Region { name } => {
                if let Some(n) = name { write!(w, "#region {}", n)?; } else { w.write_str("#region")?; }
            }
            PreprocessorDirective::EndRegion => w.write_str("#endregion")?,
            PreprocessorDirective::Error { message } => write!(w, "#error {}", message)?,
            PreprocessorDirective::Warning { message } => write!(w, "#warning {}", message)?,
            PreprocessorDirective::Pragma { pragma } => write!(w, "#pragma {}", pragma)?,
            PreprocessorDirective::Line { line } => write!(w, "#line {}", line)?,
            PreprocessorDirective::Unknown { text } => write!(w, "#{}", text)?,
        }
        Ok(())
    }
}
