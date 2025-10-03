use crate::artifacts::cfg::{ControlFlowIndex, MethodControlFlowStats};
use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::nodes::declarations::{
    ClassBodyDeclaration, ClassDeclaration, NamespaceBodyDeclaration, NamespaceDeclaration,
};
use crate::syntax::nodes::statements::statement::Statement;

pub struct ControlFlowPass;

impl AnalyzerPass for ControlFlowPass {
    fn id(&self) -> &'static str {
        "passes.control_flow"
    }
    fn phase(&self) -> Phase {
        Phase::Global
    }

    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let mut index = ControlFlowIndex::new();
        analyze_compilation_unit(cu, None, &mut index);
        session.artifacts.insert(index);
    }
}

fn analyze_compilation_unit(
    cu: &CompilationUnit,
    ns_path: Option<String>,
    index: &mut ControlFlowIndex,
) {
    for decl in &cu.declarations {
        match decl {
            TopLevelDeclaration::Namespace(ns) => {
                analyze_namespace(ns, ns.name.name.clone(), index)
            }
            TopLevelDeclaration::Class(class) => {
                analyze_class(class, ns_path.as_deref(), &mut Vec::new(), index)
            }
            _ => {}
        }
    }
}

fn analyze_namespace(ns: &NamespaceDeclaration, ns_path: String, index: &mut ControlFlowIndex) {
    for member in &ns.declarations {
        match member {
            NamespaceBodyDeclaration::Namespace(inner) => {
                analyze_namespace(inner, format!("{}.{}", ns_path, inner.name.name), index)
            }
            NamespaceBodyDeclaration::Class(class) => {
                analyze_class(class, Some(&ns_path), &mut Vec::new(), index)
            }
            NamespaceBodyDeclaration::Struct(_)
            | NamespaceBodyDeclaration::Interface(_)
            | NamespaceBodyDeclaration::Enum(_)
            | NamespaceBodyDeclaration::Delegate(_)
            | NamespaceBodyDeclaration::Record(_)
            | NamespaceBodyDeclaration::GlobalAttribute(_) => {}
        }
    }
}

fn analyze_class(
    class: &ClassDeclaration,
    ns_path: Option<&str>,
    class_stack: &mut Vec<String>,
    index: &mut ControlFlowIndex,
) {
    class_stack.push(class.name.name.clone());
    for member in &class.body_declarations {
        if let ClassBodyDeclaration::Method(m) = member {
            let complexity = calc_complexity_stmt(m.body.as_ref());
            let max_nesting = calc_max_nesting(m.body.as_ref(), 0);
            let exit_points = count_exit_points(m.body.as_ref());
            let statement_count = count_statements(m.body.as_ref());
            let class_path = class_stack.join(".");
            let fqn_key = if let Some(ns) = ns_path {
                format!("{}.{}::{}", ns, class_path, m.name.name)
            } else {
                format!("{}::{}", class_path, m.name.name)
            };
            let stats = MethodControlFlowStats {
                complexity,
                max_nesting,
                exit_points,
                statement_count,
            };
            index.insert(fqn_key, stats);
        } else if let ClassBodyDeclaration::NestedClass(nested) = member {
            analyze_class(nested, ns_path, class_stack, index);
        }
    }
    class_stack.pop();
}

fn calc_complexity_stmt(stmt: Option<&Statement>) -> usize {
    match stmt {
        None => 1,
        Some(s) => 1 + decision_points(s),
    }
}

fn decision_points(stmt: &Statement) -> usize {
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
            // Count try/catch as a decision point, and traverse try, catches, and finally blocks
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

fn calc_max_nesting(stmt: Option<&Statement>, current: usize) -> usize {
    match stmt {
        None => current,
        Some(s) => max_nesting_of(s, current),
    }
}

fn max_nesting_of(stmt: &Statement, current: usize) -> usize {
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

fn count_exit_points(stmt: Option<&Statement>) -> usize {
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

fn count_statements(stmt: Option<&Statement>) -> usize {
    match stmt {
        None => 0,
        Some(Statement::Block(stmts)) => stmts.iter().map(|s| count_statements(Some(s))).sum(),
        Some(_) => 1,
    }
}
