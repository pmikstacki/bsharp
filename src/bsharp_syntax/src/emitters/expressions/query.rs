use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::{
    FromClause, JoinClause, LetClause, OrderByOrdering, OrderingDirection, QueryClause,
    QueryContinuation, QueryExpression, QueryOrderByClause, QuerySelectOrGroup, QueryWhereClause,
};

impl Emit for QueryExpression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.from.emit(w, cx)?;
        for c in &self.body {
            w.write_char(' ')?;
            c.emit(w, cx)?;
        }
        w.write_char(' ')?;
        self.select_or_group.emit(w, cx)?;
        if let Some(cont) = &self.continuation {
            w.write_char(' ')?;
            cont.emit(w, cx)?;
        }
        Ok(())
    }
}

impl Emit for FromClause {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("from ")?;
        if let Some(t) = &self.type_annotation {
            write!(w, "{} ", t)?;
        }
        write!(w, "{} in ", self.identifier)?;
        self.expression.emit(w, cx)
    }
}

impl Emit for QueryClause {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            QueryClause::From(f) => f.emit(w, cx),
            QueryClause::Let(l) => l.emit(w, cx),
            QueryClause::Where(wc) => wc.emit(w, cx),
            QueryClause::Join(j) => j.emit(w, cx),
            QueryClause::OrderBy(o) => o.emit(w, cx),
        }
    }
}

impl Emit for LetClause {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        write!(w, "let {} = ", self.identifier)?;
        self.expression.emit(w, cx)
    }
}

impl Emit for QueryWhereClause {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("where ")?;
        self.condition.emit(w, cx)
    }
}

impl Emit for JoinClause {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("join ")?;
        if let Some(t) = &self.type_annotation {
            write!(w, "{} ", t)?;
        }
        write!(w, "{} in ", self.identifier)?;
        self.in_expression.emit(w, cx)?;
        w.write_str(" on ")?;
        self.on_expression.emit(w, cx)?;
        w.write_str(" equals ")?;
        self.equals_expression.emit(w, cx)?;
        if let Some(into) = &self.into_identifier {
            write!(w, " into {}", into)?;
        }
        Ok(())
    }
}

impl Emit for QueryOrderByClause {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        w.write_str("orderby ")?;
        for (i, o) in self.orderings.iter().enumerate() {
            if i != 0 {
                w.write_str(", ")?;
            }
            o.emit(w, cx)?;
        }
        Ok(())
    }
}

impl Emit for OrderByOrdering {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        self.expression.emit(w, cx)?;
        if let Some(dir) = &self.direction {
            w.write_char(' ')?;
            dir.emit(w, cx)?;
        }
        Ok(())
    }
}

impl Emit for OrderingDirection {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, _cx: &mut EmitCtx) -> Result<(), EmitError> {
        match self {
            OrderingDirection::Ascending => w.write_str("ascending"),
            OrderingDirection::Descending => w.write_str("descending"),
        }
        .map_err(EmitError::from)
    }
}

impl Emit for QuerySelectOrGroup {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            QuerySelectOrGroup::Select(e) => {
                w.write_str("select ")?;
                e.emit(w, cx)
            }
            QuerySelectOrGroup::Group { element, by } => {
                w.write_str("group ")?;
                element.emit(w, cx)?;
                w.write_str(" by ")?;
                by.emit(w, cx)
            }
        }
    }
}

impl Emit for QueryContinuation {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        write!(w, "into {}", self.identifier)?;
        for c in &self.body {
            w.write_char(' ')?;
            c.emit(w, cx)?;
        }
        w.write_char(' ')?;
        self.select_or_group.emit(w, cx)
    }
}
