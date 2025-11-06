use crate::framework::{AnalysisSession, AnalyzerPass, Phase, Query};
use crate::metrics::AstAnalysis;
use crate::metrics::shared::compute_statement_metrics;
use crate::syntax::ast::CompilationUnit;
use crate::syntax::declarations::{
    ClassBodyDeclaration, ClassDeclaration, StructBodyDeclaration, StructDeclaration,
};
use bsharp_syntax::declarations::{
    DelegateDeclaration, EnumDeclaration, InterfaceDeclaration, RecordDeclaration,
};

pub struct MetricsPass;

impl AnalyzerPass for MetricsPass {
    fn id(&self) -> &'static str {
        "passes.metrics"
    }
    fn phase(&self) -> Phase {
        Phase::LocalRules
    }
    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let mut analysis = AstAnalysis::default();

        // Count declaration kinds via Query
        for _ in Query::from(cu).of::<InterfaceDeclaration>() {
            analysis.total_interfaces += 1;
        }
        for _ in Query::from(cu).of::<EnumDeclaration>() {
            analysis.total_enums += 1;
        }
        for _ in Query::from(cu).of::<RecordDeclaration>() {
            analysis.total_records += 1;
        }
        for _ in Query::from(cu).of::<DelegateDeclaration>() {
            analysis.total_delegates += 1;
        }
        for c in Query::from(cu).of::<ClassDeclaration>() {
            process_class(&mut analysis, c);
        }
        for s in Query::from(cu).of::<StructDeclaration>() {
            process_struct(&mut analysis, s);
        }

        // Also consider top-level statements
        for s in &cu.top_level_statements {
            let m = compute_statement_metrics(s);
            analysis.total_if_statements += m.total_if_statements;
            analysis.total_for_loops += m.total_for_loops;
            analysis.total_while_loops += m.total_while_loops;
            analysis.total_switch_statements += m.total_switch_statements;
            analysis.total_try_statements += m.total_try_statements;
            analysis.total_using_statements += m.total_using_statements;
            analysis.cyclomatic_complexity += m.decision_points + m.extra_try_bonus;
            analysis.max_nesting_depth = analysis.max_nesting_depth.max(m.max_nesting);
            analysis.lines_of_code += m.statement_count;
        }

        // Simple LOC from source (non-empty, non-`//` lines)
        let src = session.ctx.source();
        let loc = src
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty() && !t.starts_with("//")
            })
            .count();
        analysis.lines_of_code += loc;

        session.artifacts.insert(analysis);
    }
}

fn process_class(analysis: &mut AstAnalysis, class: &ClassDeclaration) {
    analysis.total_classes += 1;
    for m in &class.body_declarations {
        match m {
            ClassBodyDeclaration::Method(m) => {
                analysis.total_methods += 1;
                analysis.cyclomatic_complexity += 1; // baseline per method
                if let Some(body) = &m.body {
                    let m = compute_statement_metrics(body);
                    analysis.total_if_statements += m.total_if_statements;
                    analysis.total_for_loops += m.total_for_loops;
                    analysis.total_while_loops += m.total_while_loops;
                    analysis.total_switch_statements += m.total_switch_statements;
                    analysis.total_try_statements += m.total_try_statements;
                    analysis.total_using_statements += m.total_using_statements;
                    analysis.max_nesting_depth = analysis.max_nesting_depth.max(m.max_nesting);
                    analysis.lines_of_code += m.statement_count;
                    analysis.cyclomatic_complexity += m.decision_points + m.extra_try_bonus;
                }
            }
            ClassBodyDeclaration::Constructor(_) => analysis.total_constructors += 1,
            ClassBodyDeclaration::Property(_) => analysis.total_properties += 1,
            ClassBodyDeclaration::Field(_) => analysis.total_fields += 1,
            ClassBodyDeclaration::Event(_) => analysis.total_events += 1,
            _ => {}
        }
    }
}

fn process_struct(analysis: &mut AstAnalysis, strukt: &StructDeclaration) {
    analysis.total_structs += 1;
    for m in &strukt.body_declarations {
        match m {
            StructBodyDeclaration::Method(m) => {
                analysis.total_methods += 1;
                analysis.cyclomatic_complexity += 1;
                if let Some(body) = &m.body {
                    let m = compute_statement_metrics(body);
                    analysis.total_if_statements += m.total_if_statements;
                    analysis.total_for_loops += m.total_for_loops;
                    analysis.total_while_loops += m.total_while_loops;
                    analysis.total_switch_statements += m.total_switch_statements;
                    analysis.total_try_statements += m.total_try_statements;
                    analysis.total_using_statements += m.total_using_statements;
                    analysis.max_nesting_depth = analysis.max_nesting_depth.max(m.max_nesting);
                    analysis.lines_of_code += m.statement_count;
                    analysis.cyclomatic_complexity += m.decision_points + m.extra_try_bonus;
                }
            }
            StructBodyDeclaration::Constructor(_) => analysis.total_constructors += 1,
            StructBodyDeclaration::Property(_) => analysis.total_properties += 1,
            StructBodyDeclaration::Field(_) => analysis.total_fields += 1,
            StructBodyDeclaration::Event(_) => analysis.total_events += 1,
            _ => {}
        }
    }
}
