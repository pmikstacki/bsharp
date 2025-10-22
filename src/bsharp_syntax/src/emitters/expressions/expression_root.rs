use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::expressions::Expression;

impl Emit for Expression {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        match self {
            Expression::AnonymousObject(x) => x.emit(w, cx),
            Expression::Tuple(x) => x.emit(w, cx),
            Expression::Range(x) => x.emit(w, cx),
            Expression::Index(x) => x.emit(w, cx),
            Expression::Pattern(p) => p.emit(w, cx),
            Expression::Deconstruction(x) => x.emit(w, cx),
            Expression::Conditional(x) => x.emit(w, cx),
            Expression::New(x) => x.emit(w, cx),
            Expression::MemberAccess(x) => x.emit(w, cx),
            Expression::NullConditional(x) => x.emit(w, cx),
            Expression::Invocation(x) => x.emit(w, cx),
            Expression::Assignment(x) => x.emit(w, cx),
            Expression::Literal(l) => l.emit(w, cx),
            Expression::Variable(id) => write!(w, "{}", id).map_err(EmitError::from),
            Expression::Unary { op, expr } => {
                op.emit(w, cx)?;
                expr.emit(w, cx)
            }
            Expression::Binary { left, op, right } => {
                left.emit(w, cx)?;
                cx.space(w)?;
                op.emit(w, cx)?;
                cx.space(w)?;
                right.emit(w, cx)
            }
            Expression::Indexing(x) => x.emit(w, cx),
            Expression::PostfixUnary { op, expr } => {
                expr.emit(w, cx)?;
                op.emit(w, cx)
            }
            Expression::This => w.write_str("this").map_err(EmitError::from),
            Expression::Base => w.write_str("base").map_err(EmitError::from),
            Expression::Lambda(x) => x.emit(w, cx),
            Expression::AnonymousMethod(x) => x.emit(w, cx),
            Expression::Await(x) => x.emit(w, cx),
            Expression::Query(x) => x.emit(w, cx),
            Expression::SwitchExpression(x) => x.emit(w, cx),
            Expression::IsPattern {
                expression,
                pattern,
            } => {
                expression.emit(w, cx)?;
                cx.space(w)?;
                w.write_str("is").map_err(EmitError::from)?;
                cx.space(w)?;
                pattern.emit(w, cx)
            }
            Expression::As {
                expression,
                target_type,
            } => {
                expression.emit(w, cx)?;
                cx.space(w)?;
                w.write_str("as").map_err(EmitError::from)?;
                cx.space(w)?;
                write!(w, "{}", target_type).map_err(EmitError::from)
            }
            Expression::Cast {
                expression,
                target_type,
            } => {
                w.write_char('(').map_err(EmitError::from)?;
                write!(w, "{}", target_type).map_err(EmitError::from)?;
                w.write_char(')').map_err(EmitError::from)?;
                expression.emit(w, cx)
            }
            Expression::Throw(x) => x.emit(w, cx),
            Expression::Nameof(x) => x.emit(w, cx),
            Expression::Typeof(x) => x.emit(w, cx),
            Expression::Sizeof(x) => x.emit(w, cx),
            Expression::Default(x) => x.emit(w, cx),
            Expression::StackAlloc(x) => x.emit(w, cx),
            Expression::Ref(expr) => {
                w.write_str("ref ").map_err(EmitError::from)?;
                expr.emit(w, cx)
            }
            Expression::Checked(x) => x.emit(w, cx),
            Expression::Unchecked(x) => x.emit(w, cx),
            Expression::With {
                target,
                initializers,
            } => {
                target.emit(w, cx)?;
                cx.space(w)?;
                w.write_str("with").map_err(EmitError::from)?;
                cx.space(w)?;
                w.write_char('{').map_err(EmitError::from)?;
                if !initializers.is_empty() {
                    cx.space(w)?;
                    for (i, init) in initializers.iter().enumerate() {
                        if i != 0 {
                            w.write_str(", ").map_err(EmitError::from)?;
                        }
                        init.emit(w, cx)?;
                    }
                    cx.space(w)?;
                }
                w.write_char('}').map_err(EmitError::from)
            }
            Expression::Collection(elements) => {
                w.write_char('[').map_err(EmitError::from)?;
                for (i, el) in elements.iter().enumerate() {
                    if i != 0 {
                        w.write_str(", ").map_err(EmitError::from)?;
                    }
                    el.emit(w, cx)?;
                }
                w.write_char(']').map_err(EmitError::from)
            }
        }
    }
}
