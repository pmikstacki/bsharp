use crate::artifacts::control_flow_graph::graph::{build_cfg, ControlFlowGraphs};
use crate::artifacts::control_flow_graph::index::ControlFlowIndex;
use crate::artifacts::control_flow_graph::stats::MethodControlFlowStats;
use crate::framework::{method_fqn, AnalysisSession, AnalyzerPass, Phase, Query};
use crate::metrics::shared::{
    count_exit_points, count_statements, decision_points, max_nesting_of,
};
use crate::syntax::ast::CompilationUnit;
use bsharp_syntax::declarations::MethodDeclaration;
use bsharp_syntax::statements::statement::Statement;

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
        let mut graphs = ControlFlowGraphs::default();

        for m in Query::from(cu).of::<MethodDeclaration>() {
            let fqn_key = method_fqn(cu, m);

            let complexity = calc_complexity_stmt(m.body.as_ref());
            let max_nesting = calc_max_nesting(m.body.as_ref(), 0);
            let exit_points = count_exit_points(m.body.as_ref());
            let statement_count = count_statements(m.body.as_ref());
            let stats = MethodControlFlowStats {
                complexity,
                max_nesting,
                exit_points,
                statement_count,
            };
            index.insert(fqn_key.clone(), stats);
            if let Some(body) = &m.body {
                let cfg = build_cfg(body);
                graphs.0.insert(fqn_key, cfg);
            }
        }

        session.insert_artifact(index);
        session.insert_artifact(graphs);
    }
}

fn calc_complexity_stmt(stmt: Option<&Statement>) -> usize {
    match stmt {
        None => 1,
        Some(s) => 1 + decision_points(s),
    }
}

fn calc_max_nesting(stmt: Option<&Statement>, current: usize) -> usize {
    match stmt {
        None => current,
        Some(s) => max_nesting_of(s, current),
    }
}
