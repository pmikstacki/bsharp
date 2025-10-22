use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::expression::{CollectionElement, WithInitializerEntry};
use crate::expressions::new_expression::ObjectInitializerEntry;

impl Emit for WithInitializerEntry {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            WithInitializerEntry::Property { name, value } => {
                write!(w, "{} = ", name)?;
                value.emit(w, cx)
            }
            WithInitializerEntry::Indexer { indices, value } => {
                w.write_char('[')?;
                for (i, idx) in indices.iter().enumerate() {
                    if i != 0 {
                        w.write_str(", ")?;
                    }
                    idx.emit(w, cx)?;
                }
                w.write_str("] = ")?;
                value.emit(w, cx)
            }
        }
    }
}

impl Emit for CollectionElement {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            CollectionElement::Expr(e) => e.emit(w, cx),
            CollectionElement::Spread(e) => {
                w.write_str("..")?;
                e.emit(w, cx)
            }
        }
    }
}

impl Emit for ObjectInitializerEntry {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            ObjectInitializerEntry::Property { name, value } => {
                write!(w, "{} = ", name)?;
                value.emit(w, cx)
            }
            ObjectInitializerEntry::Indexer { indices, value } => {
                w.write_char('[')?;
                for (i, idx) in indices.iter().enumerate() {
                    if i != 0 {
                        w.write_str(", ")?;
                    }
                    idx.emit(w, cx)?;
                }
                w.write_str("] = ")?;
                value.emit(w, cx)
            }
        }
    }
}
