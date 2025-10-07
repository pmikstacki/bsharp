use crate::syntax::nodes::{expressions::expression::Expression, statements::statement::Statement};
use crate::AstNavigate;

impl AstNavigate for Statement {
    fn find_if_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_if_statements(self, &mut results);
        results
    }

    fn find_for_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_for_loops(self, &mut results);
        results
    }

    fn find_while_loops(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_while_loops(self, &mut results);
        results
    }

    fn find_switch_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_switch_statements(self, &mut results);
        results
    }

    fn find_try_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_try_statements(self, &mut results);
        results
    }

    fn find_using_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_using_statements(self, &mut results);
        results
    }

    fn find_expressions<F>(&self, predicate: F) -> Vec<&Expression>
    where
        F: Fn(&Expression) -> bool,
    {
        let mut results = Vec::new();
        collect_expressions(self, &predicate, &mut results);
        results
    }
}

// Private recursive helpers (duplicated from the old monolithic file, minimized)
fn collect_if_statements<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::If(if_stmt) => {
            results.push(stmt);
            collect_if_statements(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_if_statements(alt, results);
            }
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_if_statements(s, results);
            }
        }
        Statement::For(for_stmt) => collect_if_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_if_statements(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_if_statements(&do_while_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_if_statements(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_for_loops<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::For(_) => {
            results.push(stmt);
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_for_loops(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_for_loops(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_for_loops(alt, results);
            }
        }
        Statement::While(while_stmt) => collect_for_loops(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_for_loops(&do_while_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_for_loops(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_while_loops<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::While(_) | Statement::DoWhile(_) => {
            results.push(stmt);
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_while_loops(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_while_loops(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_while_loops(alt, results);
            }
        }
        Statement::For(for_stmt) => collect_while_loops(&for_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_while_loops(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_switch_statements<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::Switch(_) => {
            results.push(stmt);
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_switch_statements(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_switch_statements(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_switch_statements(alt, results);
            }
        }
        Statement::For(for_stmt) => collect_switch_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_switch_statements(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => {
            collect_switch_statements(&do_while_stmt.body, results)
        }
        _ => {}
    }
}

fn collect_try_statements<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::Try(_) => {
            results.push(stmt);
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_try_statements(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_try_statements(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_try_statements(alt, results);
            }
        }
        Statement::For(for_stmt) => collect_try_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_try_statements(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_try_statements(&do_while_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_try_statements(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_using_statements<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::Using(_) => {
            results.push(stmt);
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_using_statements(s, results);
            }
        }
        Statement::If(if_stmt) => {
            collect_using_statements(&if_stmt.consequence, results);
            if let Some(alt) = &if_stmt.alternative {
                collect_using_statements(alt, results);
            }
        }
        Statement::For(for_stmt) => collect_using_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_using_statements(&while_stmt.body, results),
        Statement::DoWhile(do_while_stmt) => collect_using_statements(&do_while_stmt.body, results),
        Statement::Switch(switch_stmt) => {
            for section in &switch_stmt.sections {
                for s in &section.statements {
                    collect_using_statements(s, results);
                }
            }
        }
        _ => {}
    }
}

fn collect_expressions<'a, F>(stmt: &'a Statement, predicate: &F, results: &mut Vec<&'a Expression>)
where
    F: Fn(&Expression) -> bool,
{
    // Minimal traversal – can be expanded as needed
    match stmt {
        Statement::Expression(e) => {
            if predicate(e) {
                results.push(e);
            }
        }
        Statement::Block(stmts) => {
            for s in stmts {
                collect_expressions(s, predicate, results);
            }
        }
        Statement::If(s) => {
            if predicate(&s.condition) {
                results.push(&s.condition);
            }
            collect_expressions(&s.consequence, predicate, results);
            if let Some(alt) = &s.alternative {
                collect_expressions(alt, predicate, results);
            }
        }
        _ => {}
    }
}
