use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{Pattern, PatternDesignation, PropertySubpattern, ListPatternElement, RelationalOperator, PatternCase};

impl Emit for Pattern {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        match self {
            Pattern::Declaration { target_type, name } => { write!(w, "{} {}", target_type, name)?; }
            Pattern::Constant(e) => { e.emit(w, cx)?; }
            Pattern::Var(id) => { write!(w, "var {}", id)?; }
            Pattern::Discard => { w.write_str("_")?; }
            Pattern::Type { target_type, designation } => {
                write!(w, "{}", target_type)?;
                if let Some(d) = designation { w.write_char(' ')?; d.emit(w, cx)?; }
            }
            Pattern::Property { type_name, subpatterns } => {
                if let Some(t) = type_name { write!(w, "{} ", t)?; }
                w.write_char('{')?;
                for (i, sp) in subpatterns.iter().enumerate() { if i != 0 { w.write_str(", ")?; } sp.emit(w, cx)?; }
                w.write_char('}')?;
            }
            Pattern::Positional { type_name, subpatterns } => {
                if let Some(t) = type_name { write!(w, "{}", t)?; }
                w.write_char('(')?; for (i, p) in subpatterns.iter().enumerate() { if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; } w.write_char(')')?;
            }
            Pattern::Tuple(ps) => {
                w.write_char('(')?; for (i, p) in ps.iter().enumerate() { if i!=0 { w.write_str(", ")?; } p.emit(w, cx)?; } w.write_char(')')?;
            }
            Pattern::List { patterns } => {
                w.write_char('[')?; for (i, el) in patterns.iter().enumerate() { if i!=0 { w.write_str(", ")?; } el.emit(w, cx)?; } w.write_char(']')?;
            }
            Pattern::Slice { pattern } => {
                w.write_str("..")?; if let Some(p) = pattern { w.write_char(' ')?; p.emit(w, cx)?; }
            }
            Pattern::Relational { op, value } => {
                op.emit(w, cx)?; w.write_char(' ')?; value.emit(w, cx)?;
            }
            Pattern::LogicalAnd(a, b) => { a.emit(w, cx)?; w.write_str(" and ")?; b.emit(w, cx)?; }
            Pattern::LogicalOr(a, b)  => { a.emit(w, cx)?; w.write_str(" or ")?;  b.emit(w, cx)?; }
            Pattern::Not(p)           => { w.write_str("not ")?; p.emit(w, cx)?; }
            Pattern::Parenthesized(p) => { w.write_char('(')?; p.emit(w, cx)?; w.write_char(')')?; }
        }
        Ok(())
    }
}

impl Emit for PatternDesignation {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{
        match self {
            PatternDesignation::Variable(id) => write!(w, "{}", id)?,
            PatternDesignation::Discard => w.write_str("_")?,
            PatternDesignation::Parenthesized(inner) => { w.write_char('(')?; write!(w, "{}", "")?; w.write_char(')')?; /* printer for designation tree minimal */ },
        }
        Ok(())
    }
}

impl Emit for PropertySubpattern {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        write!(w, "{}: ", self.member_name)?; self.pattern.emit(w, cx)
    }
}

impl Emit for ListPatternElement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        match self {
            ListPatternElement::Pattern(p) => p.emit(w, cx),
            ListPatternElement::Slice(opt) => { w.write_str("..")?; if let Some(p) = opt { w.write_char(' ')?; p.emit(w, cx)?; } Ok(()) }
        }
    }
}

impl Emit for RelationalOperator { 
    fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{ 
        let s = match self { 
            RelationalOperator::LessThan => "<",
            RelationalOperator::LessThanOrEqual => "<=",
            RelationalOperator::GreaterThan => ">",
            RelationalOperator::GreaterThanOrEqual => ">=",
            RelationalOperator::Equal => "==",
            RelationalOperator::NotEqual => "!=",
        }; w.write_str(s)?; Ok(()) } }

impl Emit for PatternCase {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("case ")?; self.pattern.emit(w, cx)?; if let Some(when) = &self.when_clause { w.write_str(" when ")?; when.emit(w, cx)?; } w.write_str(": ")?; 
        // body as comma-separated expressions (placeholder, statements would be elsewhere)
        for (i, e) in self.body.iter().enumerate() { if i!=0 { w.write_str(", ")?; } e.emit(w, cx)?; }
        Ok(())
    }
}

 
