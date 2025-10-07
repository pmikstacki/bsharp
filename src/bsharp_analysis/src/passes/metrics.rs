use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::metrics::shared::{count_statements, decision_points, max_nesting_of};
use crate::metrics::AstAnalysis;
use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::nodes::declarations::ClassBodyDeclaration;

pub struct MetricsPass;

impl AnalyzerPass for MetricsPass {
    fn id(&self) -> &'static str { "passes.metrics" }
    fn phase(&self) -> Phase { Phase::LocalRules }
    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let mut analysis = AstAnalysis::default();

        for decl in &cu.declarations {
            match decl {
                TopLevelDeclaration::Class(class) => {
                    analysis.total_classes += 1;
                    for m in &class.body_declarations {
                        match m {
                            ClassBodyDeclaration::Method(m) => {
                                analysis.total_methods += 1;
                                analysis.cyclomatic_complexity += 1; // baseline per method
                                if let Some(body) = &m.body {
                                    analysis.total_if_statements += decision_points(body); // approx by decision points from ifs only
                                    analysis.max_nesting_depth = analysis
                                        .max_nesting_depth
                                        .max(max_nesting_of(body, 0));
                                    analysis.lines_of_code += count_statements(Some(body));
                                    analysis.total_try_statements += 0; // keep as part of decision points; try handled there
                                    analysis.total_for_loops += 0; // refined metrics stay in visitor for now if needed
                                    analysis.total_while_loops += 0;
                                    analysis.total_switch_statements += 0;
                                    analysis.total_using_statements += 0;
                                    analysis.cyclomatic_complexity += decision_points(body);
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
                TopLevelDeclaration::Interface(_) => analysis.total_interfaces += 1,
                TopLevelDeclaration::Struct(_) => analysis.total_structs += 1,
                TopLevelDeclaration::Record(_) => analysis.total_records += 1,
                TopLevelDeclaration::Enum(_) => analysis.total_enums += 1,
                TopLevelDeclaration::Delegate(_) => analysis.total_delegates += 1,
                _ => {}
            }
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
