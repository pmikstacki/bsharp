use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::statements::statement::Statement;

/// Depth-first walk over a statement tree, calling `f` on every statement node.
pub fn walk_statements<'a, F>(stmt: &'a Statement, f: &mut F)
where
    F: FnMut(&'a Statement),
{
    f(stmt);
    match stmt {
        // Expand as needed; keep conservative traversal to avoid missing deep nodes
        Statement::Block(statements) => {
            for s in statements {
                walk_statements(s, f);
            }
        }
        Statement::If(if_stmt) => {
            walk_statements(&if_stmt.consequence, f);
            if let Some(alt) = &if_stmt.alternative {
                walk_statements(alt, f);
            }
        }
        Statement::For(for_stmt) => walk_statements(&for_stmt.body, f),
        Statement::ForEach(for_each) => walk_statements(&for_each.body, f),
        Statement::While(while_stmt) => walk_statements(&while_stmt.body, f),
        Statement::DoWhile(do_stmt) => walk_statements(&do_stmt.body, f),
        Statement::Using(using_stmt) => {
            if let Some(body) = &using_stmt.body { walk_statements(body, f); }
        }
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    walk_statements(s, f);
                }
            }
        }
        Statement::Try(try_stmt) => {
            walk_statements(&try_stmt.try_block, f);
            for h in &try_stmt.catches { walk_statements(&h.block, f); }
            if let Some(fin) = &try_stmt.finally_clause { walk_statements(&fin.block, f); }
        }
        _ => {}
    }
}

/// Collect statements satisfying `predicate` from the subtree rooted at `stmt`.
pub fn collect_statements<'a, P>(stmt: &'a Statement, predicate: P) -> Vec<&'a Statement>
where
    P: Fn(&'a Statement) -> bool,
{
    let mut out = Vec::new();
    walk_statements(stmt, &mut |s| {
        if predicate(s) {
            out.push(s);
        }
    });
    out
}

/// Depth-first walk over an expression tree, calling `f` on every expression.
/// Note: This walker is intentionally conservative; add more variants as needed.
pub fn walk_expressions<'a, F>(expr: &'a Expression, f: &mut F)
where
    F: FnMut(&'a Expression),
{
    use Expression::*;
    f(expr);
    match expr {
        AnonymousObject(obj) => {
            for init in &obj.initializers { walk_expressions(&init.value, f); }
        }
        Tuple(t) => { for e in &t.elements { walk_expressions(e, f); } }
        Range(r) => {
            if let Some(s) = &r.start { walk_expressions(s, f); }
            if let Some(e2) = &r.end { walk_expressions(e2, f); }
        }
        Index(i) => { walk_expressions(&i.value, f); }
        Pattern(p) => {
            // Descend into pattern guards/expressions if present
            match &**p {
                crate::syntax::nodes::expressions::Pattern::Declaration { pattern: inner, .. } => {
                    if let Some(when) = inner.when_clause() { walk_expressions(when, f); }
                }
                _ => {}
            }
        }
        Deconstruction(d) => { for e in &d.expressions { walk_expressions(e, f); } }
        Conditional(c) => {
            walk_expressions(&c.condition, f);
            walk_expressions(&c.then_expr, f);
            walk_expressions(&c.else_expr, f);
        }
        New(n) => {
            for a in &n.arguments { walk_expressions(a, f); }
            if let Some(init) = &n.initializer {
                for entry in &init.entries {
                    match entry {
                        crate::syntax::nodes::expressions::WithInitializerEntry::Property { value, .. } => walk_expressions(value, f),
                        crate::syntax::nodes::expressions::WithInitializerEntry::Indexer { indices, value } => {
                            for idx in indices { walk_expressions(idx, f); }
                            walk_expressions(value, f);
                        }
                    }
                }
            }
        }
        MemberAccess(m) => { walk_expressions(&m.target, f); }
        NullConditional(nc) => { walk_expressions(&nc.target, f); }
        Invocation(inv) => {
            walk_expressions(&inv.target, f);
            for a in &inv.arguments { walk_expressions(a, f); }
        }
        Assignment(a) => {
            walk_expressions(&a.left, f);
            walk_expressions(&a.right, f);
        }
        Literal(_) | Variable(_) | This | Base => {}
        Unary { expr: e, .. } | PostfixUnary { expr: e, .. } => { walk_expressions(e, f); }
        Binary { left, right, .. } => {
            walk_expressions(left, f);
            walk_expressions(right, f);
        }
        Indexing(ix) => {
            walk_expressions(&ix.target, f);
            for idx in &ix.indices { walk_expressions(idx, f); }
        }
        Lambda(l) => {
            if let Some(body_expr) = &l.body_expression { walk_expressions(body_expr, f); }
            if let Some(body_block) = &l.body_block {
                // Lambda block statements are statements; expressions inside will be traversed by callers using walk_statements
                let _ = body_block;
            }
        }
        AnonymousMethod(am) => {
            let _ = am; // Body is a statement block; rely on statement walker integration
        }
        Await(ae) => { walk_expressions(&ae.expression, f); }
        Query(q) => {
            for clause in &q.clauses { for e in clause.expressions() { walk_expressions(e, f); } }
        }
        SwitchExpression(se) => {
            walk_expressions(&se.expression, f);
            for arm in &se.arms {
                if let Some(when) = &arm.when_clause { walk_expressions(when, f); }
                walk_expressions(&arm.expression, f);
            }
        }
        IsPattern { expression, .. } => walk_expressions(expression, f),
        As { expression, .. } => walk_expressions(expression, f),
        Cast { expression, .. } => walk_expressions(expression, f),
        Throw(te) => { if let Some(e) = &te.expression { walk_expressions(e, f); } }
        Nameof(ne) => { walk_expressions(&ne.expression, f); }
        Typeof(_) => {}
        Sizeof(_) => {}
        Default(de) => { if let Some(t) = &de.value_expression { walk_expressions(t, f); } }
        StackAlloc(sa) => { for e in &sa.dimensions { walk_expressions(e, f); } }
        Ref(inner) => { walk_expressions(inner, f); }
        Checked(ce) => { walk_expressions(&ce.expression, f); }
        Unchecked(ue) => { walk_expressions(&ue.expression, f); }
        With { target, initializers } => {
            walk_expressions(target, f);
            for init in initializers {
                match init {
                    crate::syntax::nodes::expressions::WithInitializerEntry::Property { value, .. } => walk_expressions(value, f),
                    crate::syntax::nodes::expressions::WithInitializerEntry::Indexer { indices, value } => {
                        for idx in indices { walk_expressions(idx, f); }
                        walk_expressions(value, f);
                    }
                }
            }
        }
        Collection(items) => {
            for item in items {
                match item {
                    crate::syntax::nodes::expressions::CollectionElement::Expr(e) => walk_expressions(e, f),
                    crate::syntax::nodes::expressions::CollectionElement::Spread(e) => walk_expressions(e, f),
                }
            }
        }
    }
}

/// Collect expressions satisfying `predicate` from the subtree rooted at `expr`.
pub fn collect_expressions<'a, P>(expr: &'a Expression, predicate: P) -> Vec<&'a Expression>
where
    P: Fn(&'a Expression) -> bool,
{
    let mut out = Vec::new();
    walk_expressions(expr, &mut |e| {
        if predicate(e) {
            out.push(e);
        }
    });
    out
}
