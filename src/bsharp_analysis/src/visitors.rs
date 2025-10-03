use bsharp_syntax::expressions::expression::{CollectionElement, Expression, WithInitializerEntry};
use bsharp_syntax::expressions::new_expression::ObjectInitializerEntry;
use bsharp_syntax::expressions::query_expression::{QueryClause, QuerySelectOrGroup};
use bsharp_syntax::statements::statement::Statement;

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
            if let Some(body) = &using_stmt.body {
                walk_statements(body, f);
            }
        }
        Statement::Lock(lock_stmt) => {
            walk_statements(&lock_stmt.body, f);
        }
        Statement::Fixed(fixed_stmt) => {
            walk_statements(&fixed_stmt.body, f);
        }
        Statement::Unsafe(unsafe_stmt) => {
            walk_statements(&unsafe_stmt.body, f);
        }
        Statement::LocalFunction(local_fn) => {
            walk_statements(&local_fn.body, f);
        }
        Statement::Yield(_)
        | Statement::Break(_)
        | Statement::Continue(_)
        | Statement::Goto(_)
        | Statement::GotoCase(_)
        | Statement::Label(_)
        | Statement::Declaration(_)
        | Statement::Expression(_)
        | Statement::Return(_)
        | Statement::Throw(_)
        | Statement::Empty
        | Statement::Deconstruction(_) => { /* leaf or handled by expression walker elsewhere */ }
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    walk_statements(s, f);
                }
            }
        }
        Statement::Try(try_stmt) => {
            walk_statements(&try_stmt.try_block, f);
            for h in &try_stmt.catches {
                walk_statements(&h.block, f);
            }
            if let Some(fin) = &try_stmt.finally_clause {
                walk_statements(&fin.block, f);
            }
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
            for init in &obj.initializers {
                walk_expressions(&init.value, f);
            }
        }
        Tuple(t) => {
            for e in &t.elements {
                walk_expressions(&e.value, f);
            }
        }
        Range(r) => {
            if let Some(s) = &r.start {
                walk_expressions(s, f);
            }
            if let Some(e2) = &r.end {
                walk_expressions(e2, f);
            }
        }
        Index(i) => {
            walk_expressions(&i.value, f);
        }
        Pattern(p) => {
            walk_pattern(p, f);
        }
        Deconstruction(d) => {
            walk_expressions(&d.value, f);
        }
        Conditional(c) => {
            walk_expressions(&c.condition, f);
            walk_expressions(&c.consequence, f);
            walk_expressions(&c.alternative, f);
        }
        New(n) => {
            for a in &n.arguments {
                walk_expressions(a, f);
            }
            if let Some(obj_inits) = &n.object_initializer {
                for entry in obj_inits {
                    match entry {
                        ObjectInitializerEntry::Property { value, .. } => {
                            walk_expressions(value, f)
                        }
                        ObjectInitializerEntry::Indexer { indices, value } => {
                            for idx in indices {
                                walk_expressions(idx, f);
                            }
                            walk_expressions(value, f);
                        }
                    }
                }
            }
            if let Some(col_inits) = &n.collection_initializer {
                for e in col_inits {
                    walk_expressions(e, f);
                }
            }
        }
        MemberAccess(m) => {
            walk_expressions(&m.object, f);
        }
        NullConditional(nc) => {
            walk_expressions(&nc.target, f);
        }
        Invocation(inv) => {
            walk_expressions(&inv.callee, f);
            for a in &inv.arguments {
                walk_expressions(&a.expr, f);
            }
        }
        Assignment(a) => {
            walk_expressions(&a.target, f);
            walk_expressions(&a.value, f);
        }
        Literal(_) | Variable(_) | This | Base => {}
        Unary { expr: e, .. } | PostfixUnary { expr: e, .. } => {
            walk_expressions(e, f);
        }
        Binary { left, right, .. } => {
            walk_expressions(left, f);
            walk_expressions(right, f);
        }
        Indexing(ix) => {
            walk_expressions(&ix.target, f);
            walk_expressions(&ix.index, f);
        }
        Lambda(l) => {
            match &l.body {
                bsharp_syntax::expressions::lambda_expression::LambdaBody::ExpressionSyntax(
                    body_expr,
                ) => {
                    walk_expressions(body_expr, f);
                }
                bsharp_syntax::expressions::lambda_expression::LambdaBody::Block(_stmts) => {
                    // handled by statement walker when integrated
                }
            }
        }
        AnonymousMethod(am) => {
            let _ = am; // Body is a statement block; rely on statement walker integration
        }
        Await(ae) => {
            walk_expressions(&ae.expr, f);
        }
        Query(q) => {
            // from
            walk_expressions(&q.from.expression, f);
            // body clauses
            for clause in &q.body {
                match clause {
                    QueryClause::From(c) => walk_expressions(&c.expression, f),
                    QueryClause::Let(c) => walk_expressions(&c.expression, f),
                    QueryClause::Where(c) => walk_expressions(&c.condition, f),
                    QueryClause::Join(c) => {
                        walk_expressions(&c.in_expression, f);
                        walk_expressions(&c.on_expression, f);
                        walk_expressions(&c.equals_expression, f);
                    }
                    QueryClause::OrderBy(c) => {
                        for ord in &c.orderings {
                            walk_expressions(&ord.expression, f);
                        }
                    }
                }
            }
            // select or group
            match &q.select_or_group {
                QuerySelectOrGroup::Select(e) => walk_expressions(e, f),
                QuerySelectOrGroup::Group { element, by } => {
                    walk_expressions(element, f);
                    walk_expressions(by, f);
                }
            }
            // continuation
            if let Some(cont) = &q.continuation {
                for clause in &cont.body {
                    match clause {
                        QueryClause::From(c) => walk_expressions(&c.expression, f),
                        QueryClause::Let(c) => walk_expressions(&c.expression, f),
                        QueryClause::Where(c) => walk_expressions(&c.condition, f),
                        QueryClause::Join(c) => {
                            walk_expressions(&c.in_expression, f);
                            walk_expressions(&c.on_expression, f);
                            walk_expressions(&c.equals_expression, f);
                        }
                        QueryClause::OrderBy(c) => {
                            for ord in &c.orderings {
                                walk_expressions(&ord.expression, f);
                            }
                        }
                    }
                }
                match &cont.select_or_group {
                    QuerySelectOrGroup::Select(e) => walk_expressions(e, f),
                    QuerySelectOrGroup::Group { element, by } => {
                        walk_expressions(element, f);
                        walk_expressions(by, f);
                    }
                }
            }
        }
        SwitchExpression(se) => {
            walk_expressions(&se.expression, f);
            for arm in &se.arms {
                if let Some(when) = &arm.when_clause {
                    walk_expressions(when, f);
                }
                walk_expressions(&arm.expression, f);
            }
        }
        IsPattern { expression, .. } => walk_expressions(expression, f),
        As { expression, .. } => walk_expressions(expression, f),
        Cast { expression, .. } => walk_expressions(expression, f),
        Throw(te) => {
            if let Some(e) = &te.expr {
                walk_expressions(e, f);
            }
        }
        Nameof(ne) => {
            walk_expressions(&ne.expr, f);
        }
        Typeof(_) => {}
        Sizeof(_) => {}
        Default(_) => {}
        StackAlloc(sa) => {
            if let Some(c) = &sa.count {
                walk_expressions(c, f);
            }
            if let Some(init) = &sa.initializer {
                for e in init {
                    walk_expressions(e, f);
                }
            }
        }
        Ref(inner) => {
            walk_expressions(inner, f);
        }
        Checked(ce) => {
            walk_expressions(&ce.expr, f);
        }
        Unchecked(ue) => {
            walk_expressions(&ue.expr, f);
        }
        With {
            target,
            initializers,
        } => {
            walk_expressions(target, f);
            for init in initializers {
                match init {
                    WithInitializerEntry::Property { value, .. } => walk_expressions(value, f),
                    WithInitializerEntry::Indexer { indices, value } => {
                        for idx in indices {
                            walk_expressions(idx, f);
                        }
                        walk_expressions(value, f);
                    }
                }
            }
        }
        Collection(items) => {
            for item in items {
                match item {
                    CollectionElement::Expr(e) => walk_expressions(e, f),
                    CollectionElement::Spread(e) => walk_expressions(e, f),
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

fn walk_pattern<'a, F>(pat: &'a bsharp_syntax::expressions::pattern::Pattern, f: &mut F)
where
    F: FnMut(&'a Expression),
{
    use bsharp_syntax::expressions::pattern::{ListPatternElement, Pattern};
    match pat {
        Pattern::Declaration { .. } => {}
        Pattern::Constant(expr) => walk_expressions(expr, f),
        Pattern::Var(_) => {}
        Pattern::Discard => {}
        Pattern::Type { designation, .. } => {
            if let Some(des) = designation {
                match des {
                    bsharp_syntax::expressions::pattern::PatternDesignation::Variable(_) => {}
                    bsharp_syntax::expressions::pattern::PatternDesignation::Discard => {}
                    bsharp_syntax::expressions::pattern::PatternDesignation::Parenthesized(
                        inner,
                    ) => {
                        // Only designations; no embedded expressions to visit
                        let _ = inner;
                    }
                }
            }
        }
        Pattern::Property { subpatterns, .. } => {
            for sp in subpatterns {
                walk_pattern(&sp.pattern, f);
            }
        }
        Pattern::Positional { subpatterns, .. } => {
            for p in subpatterns {
                walk_pattern(p, f);
            }
        }
        Pattern::Tuple(items) => {
            for p in items {
                walk_pattern(p, f);
            }
        }
        Pattern::List { patterns } => {
            for el in patterns {
                match el {
                    ListPatternElement::Pattern(p) => walk_pattern(p, f),
                    ListPatternElement::Slice(opt) => {
                        if let Some(p) = opt {
                            walk_pattern(p, f);
                        }
                    }
                }
            }
        }
        Pattern::Slice { pattern } => {
            if let Some(p) = pattern.as_deref() {
                walk_pattern(p, f);
            }
        }
        Pattern::Relational { value, .. } => walk_expressions(value, f),
        Pattern::LogicalAnd(a, b) => {
            walk_pattern(a, f);
            walk_pattern(b, f);
        }
        Pattern::LogicalOr(a, b) => {
            walk_pattern(a, f);
            walk_pattern(b, f);
        }
        Pattern::Not(p) => walk_pattern(p, f),
        Pattern::Parenthesized(p) => walk_pattern(p, f),
    }
}
