use crate::syntax::statements::statement::Statement;

pub fn walk_statement(s: &Statement, f: &mut impl FnMut(&Statement)) {
    f(s);
    match s {
        Statement::Block(stmts) => {
            for st in stmts {
                walk_statement(st, f);
            }
        }
        Statement::If(s1) => {
            walk_statement(&s1.consequence, f);
            if let Some(alt) = &s1.alternative {
                walk_statement(alt, f);
            }
        }
        Statement::For(s1) => walk_statement(&s1.body, f),
        Statement::ForEach(s1) => walk_statement(&s1.body, f),
        Statement::While(s1) => walk_statement(&s1.body, f),
        Statement::DoWhile(s1) => walk_statement(&s1.body, f),
        Statement::Switch(sw) => {
            for sec in &sw.sections {
                for st in &sec.statements {
                    walk_statement(st, f);
                }
            }
        }
        Statement::Try(t) => {
            walk_statement(&t.try_block, f);
            for c in &t.catches {
                walk_statement(&c.block, f);
            }
            if let Some(fin) = &t.finally_clause {
                walk_statement(&fin.block, f);
            }
        }
        _ => {}
    }
}

pub trait StmtHooks {
    fn enter(&mut self, _s: &Statement) {}
    fn exit(&mut self, _s: &Statement) {}
}

pub fn walk_statement_with(h: &mut impl StmtHooks, s: &Statement) {
    h.enter(s);
    match s {
        Statement::Block(stmts) => {
            for st in stmts {
                walk_statement_with(h, st);
            }
        }
        Statement::If(s1) => {
            walk_statement_with(h, &s1.consequence);
            if let Some(alt) = &s1.alternative {
                walk_statement_with(h, alt);
            }
        }
        Statement::For(s1) => walk_statement_with(h, &s1.body),
        Statement::ForEach(s1) => walk_statement_with(h, &s1.body),
        Statement::While(s1) => walk_statement_with(h, &s1.body),
        Statement::DoWhile(s1) => walk_statement_with(h, &s1.body),
        Statement::Switch(sw) => {
            for sec in &sw.sections {
                for st in &sec.statements {
                    walk_statement_with(h, st);
                }
            }
        }
        Statement::Try(t) => {
            walk_statement_with(h, &t.try_block);
            for c in &t.catches {
                walk_statement_with(h, &c.block);
            }
            if let Some(fin) = &t.finally_clause {
                walk_statement_with(h, &fin.block);
            }
        }
        _ => {}
    }
    h.exit(s);
}
