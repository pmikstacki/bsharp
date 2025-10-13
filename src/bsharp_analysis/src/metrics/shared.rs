use crate::syntax::statements::statement::Statement;

pub fn decision_points(stmt: &Statement) -> usize {
    match stmt {
        Statement::If(if_stmt) => {
            let mut d = 1 + decision_points(&if_stmt.consequence);
            if let Some(alt) = &if_stmt.alternative {
                d += decision_points(alt);
            }
            d
        }
        Statement::For(for_stmt) => 1 + decision_points(&for_stmt.body),
        Statement::ForEach(fe) => 1 + decision_points(&fe.body),
        Statement::While(while_stmt) => 1 + decision_points(&while_stmt.body),
        Statement::DoWhile(dw) => 1 + decision_points(&dw.body),
        Statement::Using(us) => 1 + us.body.as_ref().map(|b| decision_points(b)).unwrap_or(0),
        Statement::Switch(sw) => {
            let mut d = sw.sections.len();
            for sec in &sw.sections {
                for s in &sec.statements {
                    d += decision_points(s);
                }
            }
            d
        }
        Statement::Try(try_stmt) => {
            let mut d = 1 + decision_points(&try_stmt.try_block);
            for h in &try_stmt.catches {
                d += decision_points(&h.block);
            }
            if let Some(fin) = &try_stmt.finally_clause {
                d += decision_points(&fin.block);
            }
            d
        }
        Statement::Block(stmts) => stmts.iter().map(decision_points).sum(),
        _ => 0,
    }
}

pub fn max_nesting_of(stmt: &Statement, current: usize) -> usize {
    match stmt {
        Statement::If(if_stmt) => {
            let new_depth = current + 1;
            let c = max_nesting_of(&if_stmt.consequence, new_depth);
            let a = if let Some(alt) = &if_stmt.alternative {
                max_nesting_of(alt, new_depth)
            } else {
                new_depth
            };
            c.max(a)
        }
        Statement::For(for_stmt) => max_nesting_of(&for_stmt.body, current + 1),
        Statement::ForEach(fe) => max_nesting_of(&fe.body, current + 1),
        Statement::While(while_stmt) => max_nesting_of(&while_stmt.body, current + 1),
        Statement::DoWhile(dw) => max_nesting_of(&dw.body, current + 1),
        Statement::Using(us) => {
            if let Some(b) = &us.body {
                max_nesting_of(b, current + 1)
            } else {
                current
            }
        }
        Statement::Switch(sw) => {
            let mut max_d = current + 1;
            for sec in &sw.sections {
                for s in &sec.statements {
                    max_d = max_d.max(max_nesting_of(s, current + 1));
                }
            }
            max_d
        }
        Statement::Block(stmts) => {
            let mut max_d = current;
            for s in stmts {
                max_d = max_d.max(max_nesting_of(s, current));
            }
            max_d
        }
        _ => current,
    }
}

pub fn count_exit_points(stmt: Option<&Statement>) -> usize {
    match stmt {
        None => 0,
        Some(s) => match s {
            Statement::Return(_) | Statement::Throw(_) => 1,
            Statement::If(if_stmt) => {
                let mut c = count_exit_points(Some(&if_stmt.consequence));
                if let Some(alt) = &if_stmt.alternative {
                    c += count_exit_points(Some(alt));
                }
                c
            }
            Statement::For(for_stmt) => count_exit_points(Some(&for_stmt.body)),
            Statement::ForEach(fe) => count_exit_points(Some(&fe.body)),
            Statement::While(while_stmt) => count_exit_points(Some(&while_stmt.body)),
            Statement::DoWhile(dw) => count_exit_points(Some(&dw.body)),
            Statement::Using(us) => {
                if let Some(b) = &us.body {
                    count_exit_points(Some(b))
                } else {
                    0
                }
            }
            Statement::Switch(sw) => sw
                .sections
                .iter()
                .map(|sec| {
                    sec.statements
                        .iter()
                        .map(|s| count_exit_points(Some(s)))
                        .sum::<usize>()
                })
                .sum(),
            Statement::Try(try_stmt) => {
                let mut c = count_exit_points(Some(&try_stmt.try_block));
                for h in &try_stmt.catches {
                    c += count_exit_points(Some(&h.block));
                }
                if let Some(fin) = &try_stmt.finally_clause {
                    c += count_exit_points(Some(&fin.block));
                }
                c
            }
            Statement::Block(stmts) => stmts.iter().map(|s| count_exit_points(Some(s))).sum(),
            _ => 0,
        },
    }
}

pub fn count_statements(stmt: Option<&Statement>) -> usize {
    match stmt {
        None => 0,
        Some(Statement::Block(stmts)) => stmts.iter().map(|s| count_statements(Some(s))).sum(),
        Some(_) => 1,
    }
}
