use crate::analysis::framework::session::AnalysisSession;
use crate::analysis::framework::walker::{NodeRef, Visit};
use crate::analysis::metrics::core::AstAnalysis;
use crate::syntax::ast::TopLevelDeclaration;
use crate::syntax::nodes::declarations::ClassBodyDeclaration;
use crate::syntax::nodes::statements::statement::Statement;

pub struct MetricsVisitor {
    analysis: AstAnalysis,
}

impl MetricsVisitor {
    pub fn new() -> Self { Self { analysis: AstAnalysis::default() } }
}

impl Visit for MetricsVisitor {
    fn enter(&mut self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu, _ => return };
        for decl in &cu.declarations {
            match decl {
                TopLevelDeclaration::Class(class) => {
                    self.analysis.total_classes += 1;
                    // Count members
                    for m in &class.body_declarations {
                        match m {
                            ClassBodyDeclaration::Method(m) => {
                                self.analysis.total_methods += 1;
                                // Baseline complexity of 1 per method to match CFG baseline
                                self.analysis.cyclomatic_complexity += 1;
                                if let Some(body) = &m.body { count_statements(body, &mut self.analysis); }
                            }
                            ClassBodyDeclaration::Constructor(_) => self.analysis.total_constructors += 1,
                            ClassBodyDeclaration::Property(_) => self.analysis.total_properties += 1,
                            ClassBodyDeclaration::Field(_) => self.analysis.total_fields += 1,
                            ClassBodyDeclaration::Event(_) => self.analysis.total_events += 1,
                            _ => {}
                        }
                    }
                }
                TopLevelDeclaration::Interface(_) => self.analysis.total_interfaces += 1,
                TopLevelDeclaration::Struct(_) => self.analysis.total_structs += 1,
                TopLevelDeclaration::Record(_) => self.analysis.total_records += 1,
                TopLevelDeclaration::Enum(_) => self.analysis.total_enums += 1,
                TopLevelDeclaration::Delegate(_) => self.analysis.total_delegates += 1,
                _ => {}
            }
        }

        // Compute a simple LOC: count non-empty, non-line-comment lines
        let src = session.ctx.source();
        let loc = src
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty() && !t.starts_with("//")
            })
            .count();
        self.analysis.lines_of_code += loc;
    }

    fn exit(&mut self, node: &NodeRef, session: &mut AnalysisSession) {
        match node {
            NodeRef::CompilationUnit(_) => {
                // Store the analysis artifact for reporting phase
                session.artifacts.insert(self.analysis.clone());
            }
            _ => {}
        }
    }
}

fn count_statements(stmt: &Statement, analysis: &mut AstAnalysis) {
    use crate::syntax::nodes::statements::statement::Statement::*;
    match stmt {
        If(if_stmt) => {
            analysis.total_if_statements += 1;
            analysis.cyclomatic_complexity += 1;
            count_statements(&if_stmt.consequence, analysis);
            if let Some(alt) = &if_stmt.alternative { count_statements(alt, analysis); }
        }
        For(for_stmt) => {
            analysis.total_for_loops += 1;
            analysis.cyclomatic_complexity += 1;
            count_statements(&for_stmt.body, analysis);
        }
        While(while_stmt) => {
            analysis.total_while_loops += 1;
            analysis.cyclomatic_complexity += 1;
            count_statements(&while_stmt.body, analysis);
        }
        DoWhile(dw) => {
            analysis.cyclomatic_complexity += 1;
            count_statements(&dw.body, analysis);
        }
        Switch(sw) => {
            analysis.total_switch_statements += 1;
            analysis.cyclomatic_complexity += sw.sections.len();
            for sec in &sw.sections { for s in &sec.statements { count_statements(s, analysis); } }
        }
        Try(try_stmt) => {
            analysis.total_try_statements += 1;
            count_statements(&try_stmt.try_block, analysis);
            for h in &try_stmt.catches { count_statements(&h.block, analysis); }
            if let Some(fin) = &try_stmt.finally_clause { count_statements(&fin.block, analysis); }
        }
        Using(u) => {
            analysis.total_using_statements += 1;
            if let Some(body) = u.body.as_deref() { count_statements(body, analysis); }
        }
        Block(stmts) => { for s in stmts { count_statements(s, analysis); } }
        _ => {}
    }
}
