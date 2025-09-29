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
        Statement::While(while_stmt) => walk_statements(&while_stmt.body, f),
        Statement::DoWhile(do_stmt) => walk_statements(&do_stmt.body, f),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    walk_statements(s, f);
                }
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
    f(expr);
    // TODO: Expand traversal by matching on `Expression` variants and visiting children.
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
