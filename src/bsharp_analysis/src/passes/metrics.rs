use crate::framework::{AnalysisSession, AnalyzerPass, Phase, Query};
use crate::metrics::AstAnalysis;
use crate::metrics::shared::{count_statements, decision_points, max_nesting_of};
use crate::syntax::ast::CompilationUnit;
use crate::syntax::declarations::{
    ClassBodyDeclaration, ClassDeclaration, StructBodyDeclaration, StructDeclaration,
};
use crate::syntax::statements::statement::Statement;
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
            accumulate_statement_metrics(&mut analysis, s);
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
                    accumulate_statement_metrics(analysis, body);
                    analysis.max_nesting_depth =
                        analysis.max_nesting_depth.max(max_nesting_of(body, 0));
                    analysis.lines_of_code += count_statements(Some(body));
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

fn process_struct(analysis: &mut AstAnalysis, strukt: &StructDeclaration) {
    analysis.total_structs += 1;
    for m in &strukt.body_declarations {
        match m {
            StructBodyDeclaration::Method(m) => {
                analysis.total_methods += 1;
                analysis.cyclomatic_complexity += 1;
                if let Some(body) = &m.body {
                    accumulate_statement_metrics(analysis, body);
                    analysis.max_nesting_depth =
                        analysis.max_nesting_depth.max(max_nesting_of(body, 0));
                    analysis.lines_of_code += count_statements(Some(body));
                    analysis.cyclomatic_complexity += decision_points(body);
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

fn accumulate_statement_metrics(analysis: &mut AstAnalysis, root: &Statement) {
    fn walk(s: &Statement, f: &mut impl FnMut(&Statement)) {
        f(s);
        match s {
            Statement::Block(stmts) => {
                for st in stmts {
                    walk(st, f);
                }
            }
            Statement::If(s1) => {
                walk(&s1.consequence, f);
                if let Some(alt) = &s1.alternative {
                    walk(alt, f);
                }
            }
            Statement::For(s1) => walk(&s1.body, f),
            Statement::ForEach(s1) => walk(&s1.body, f),
            Statement::While(s1) => walk(&s1.body, f),
            Statement::DoWhile(s1) => walk(&s1.body, f),
            Statement::Switch(sw) => {
                for sec in &sw.sections {
                    for st in &sec.statements {
                        walk(st, f);
                    }
                }
            }
            Statement::Try(t) => {
                walk(&t.try_block, f);
                for c in &t.catches {
                    walk(&c.block, f);
                }
                if let Some(fin) = &t.finally_clause {
                    walk(&fin.block, f);
                }
            }
            _ => {}
        }
    }
    walk(root, &mut |s| match s {
        Statement::If(_) => analysis.total_if_statements += 1,
        Statement::For(_) | Statement::ForEach(_) => analysis.total_for_loops += 1,
        Statement::While(_) | Statement::DoWhile(_) => analysis.total_while_loops += 1,
        Statement::Switch(_) => analysis.total_switch_statements += 1,
        Statement::Try(_) => {
            analysis.total_try_statements += 1;
            analysis.cyclomatic_complexity += 1;
        }
        Statement::Using(_) => analysis.total_using_statements += 1,
        _ => {}
    });
}
