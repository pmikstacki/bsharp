use crate::framework::registry::AnalyzerRegistry;
use crate::framework::visit::Visit;
use crate::framework::{AnalysisSession, AstWalker, Phase, Rule};
use crate::report::CfgSummary;
use crate::syntax::ast::CompilationUnit;
use crate::workspace::model::{ProjectFileKind, Workspace};
use crate::{AnalysisContext, AnalysisReport, AstAnalysis, DiagnosticCollection};
use bsharp_parser::facade::Parser;
use bsharp_syntax::span::Span;
#[cfg(feature = "parallel_analysis")]
use rayon::prelude::*;

pub struct AnalyzerPipeline;

/// Shared visitor that dispatches to a set of rules on each node enter
struct RulesVisitor<'a> {
    rules: Vec<&'a dyn Rule>,
}
impl Visit for RulesVisitor<'_> {
    fn enter(&mut self, node: &crate::framework::NodeRef, session: &mut AnalysisSession) {
        for r in &self.rules {
            r.visit(node, session);
        }
    }
}

impl AnalyzerPipeline {
    /// Run the analyzer pipeline using a registry derived from the session's config
    pub fn run_with_defaults(cu: &CompilationUnit, session: &mut AnalysisSession) {
        let registry = AnalyzerRegistry::from_config(&session.config);
        Self::run_for_file(cu, session, &registry);
    }

    pub fn run_for_file(
        cu: &CompilationUnit,
        session: &mut AnalysisSession,
        registry: &AnalyzerRegistry,
    ) {
        // 1) Index phase passes (no-op if none)
        Self::run_phase(Phase::Index, cu, session, registry);
        // 2) Local (per-file) passes that produce artifacts (e.g., metrics)
        Self::run_phase(Phase::LocalRules, cu, session, registry);
        // 3) Local rules in a single traversal
        Self::run_local_rules(cu, session, registry);
        // 4) Global passes
        Self::run_phase(Phase::Global, cu, session, registry);
        // 5) Semantic passes/rules
        Self::run_semantic_rules(cu, session, registry);
        Self::run_phase(Phase::Semantic, cu, session, registry);
        // 6) Reporting
        Self::run_phase(Phase::Reporting, cu, session, registry);
    }

    fn run_phase(
        phase: Phase,
        cu: &CompilationUnit,
        session: &mut AnalysisSession,
        registry: &AnalyzerRegistry,
    ) {
        for pass in registry.passes() {
            if pass.phase() != phase {
                continue;
            }
            // Config-based toggle: if enable_passes contains an entry for this id, honor it.
            if let Some(enabled) = session.config.enable_passes.get(pass.id())
                && !enabled
            {
                continue;
            }
            pass.run(cu, session);
        }
    }

    fn run_local_rules(
        cu: &CompilationUnit,
        session: &mut AnalysisSession,
        registry: &AnalyzerRegistry,
    ) {
        let rules = Self::collect_rules(registry, false, &session.config);
        Self::run_rules_via_walk(cu, session, rules);
    }

    fn run_semantic_rules(
        cu: &CompilationUnit,
        session: &mut AnalysisSession,
        registry: &AnalyzerRegistry,
    ) {
        let rules = Self::collect_rules(registry, true, &session.config);
        Self::run_rules_via_walk(cu, session, rules);
    }

    fn collect_rules<'a, 'b>(
        registry: &'a AnalyzerRegistry,
        semantic: bool,
        cfg: &'b crate::context::AnalysisConfig,
    ) -> Vec<&'a dyn Rule> {
        let mut all: Vec<&'a dyn Rule> = Vec::new();
        let iter = if semantic {
            registry.rulesets_semantic().iter()
        } else {
            registry.rulesets_local().iter()
        };
        for rs in iter {
            if let Some(enabled) = cfg.enable_rulesets.get(rs.id) && !enabled {
                continue;
            }
            for r in rs.iter() {
                all.push(r);
            }
        }
        all
    }

    fn run_rules_via_walk(
        cu: &CompilationUnit,
        session: &mut AnalysisSession,
        rules: Vec<&dyn Rule>,
    ) {
        if rules.is_empty() {
            return;
        }
        let mut walker = AstWalker::new();
        walker = walker.with_visitor(Box::new(RulesVisitor { rules }));
        walker.run(cu, session);
    }

    pub fn analyze_file_report(
        path: &std::path::Path,
        config: Option<&crate::context::AnalysisConfig>,
    ) -> Option<AnalysisReport> {
        let path_str = path.display().to_string();
        let source = std::fs::read_to_string(path).ok()?;
        let parser = Parser::new();
        let (cu, spans) = parser.parse_with_spans(Span::new(&source)).ok()?;
        let mut ctx = AnalysisContext::new(path_str, source);
        if let Some(cfg) = config {
            ctx.config = cfg.clone();
        }
        let mut session = AnalysisSession::new(ctx.clone(), spans.clone());
        Self::run_with_defaults(&cu, &mut session);
        Some(AnalysisReport::from_session(&session))
    }

    fn merge_report(
        merged_diags: &mut DiagnosticCollection,
        merged_metrics: &mut Option<AstAnalysis>,
        merged_cfg: &mut Option<CfgSummary>,
        dep_node_keys: &mut std::collections::HashSet<String>,
        dep_edge_keys: &mut std::collections::HashSet<String>,
        report: &AnalysisReport,
    ) {
        // diagnostics
        merged_diags.extend(report.diagnostics.clone());
        // metrics
        if let Some(m) = &report.metrics {
            let m = m.clone();
            *merged_metrics = Some(match merged_metrics.take() {
                Some(prev) => prev.combine(m),
                None => m,
            });
        }
        // cfg summary
        if let Some(cfg) = &report.cfg {
            let cfg = cfg.clone();
            *merged_cfg = Some(match merged_cfg.take() {
                Some(prev) => CfgSummary {
                    total_methods: prev.total_methods + cfg.total_methods,
                    high_complexity_methods: prev.high_complexity_methods
                        + cfg.high_complexity_methods,
                    deep_nesting_methods: prev.deep_nesting_methods + cfg.deep_nesting_methods,
                },
                None => cfg,
            });
        }
        // deps
        if let Some(node_keys) = &report.deps_node_keys {
            dep_node_keys.extend(node_keys.iter().cloned());
        }
        if let Some(edge_keys) = &report.deps_edge_keys {
            dep_edge_keys.extend(edge_keys.iter().cloned());
        }
    }

    fn collect_workspace_warnings(workspace: &Workspace) -> Vec<String> {
        let mut ws_warnings: Vec<String> = Vec::new();
        if let Some(sol) = &workspace.solution {
            ws_warnings.extend(sol.errors.clone());
        }
        for p in &workspace.projects {
            ws_warnings.extend(p.errors.clone());
        }
        ws_warnings.sort();
        ws_warnings.dedup();
        ws_warnings
    }

    /// Analyze an entire workspace deterministically and return an aggregated report.
    /// Iterates projects/files sorted by absolute path, analyzes each file independently,
    /// and combines diagnostics and artifacts into a single report.
    pub fn run_workspace(workspace: &Workspace) -> AnalysisReport {
        // Collect all .cs source files deterministically
        let mut files: Vec<std::path::PathBuf> = workspace
            .projects
            .iter()
            .flat_map(|p| p.files.iter())
            .filter(|f| matches!(f.kind, ProjectFileKind::Source))
            .map(|f| f.path.clone())
            .collect();
        files.sort();
        files.dedup();

        let mut merged_diags = DiagnosticCollection::default();
        let mut merged_metrics: Option<AstAnalysis> = None;
        let mut merged_cfg: Option<CfgSummary> = None;
        let mut dep_node_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut dep_edge_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

        for path in files {
            if let Some(report) = Self::analyze_file_report(&path, None) {
                Self::merge_report(
                    &mut merged_diags,
                    &mut merged_metrics,
                    &mut merged_cfg,
                    &mut dep_node_keys,
                    &mut dep_edge_keys,
                    &report,
                );
            }
        }

        // Stable diagnostic ordering: by file, line, column, then code string
        Self::sort_diagnostics(&mut merged_diags);
        // Merge workspace-level loader errors as warnings (deterministic ordering)
        let ws_warnings = Self::collect_workspace_warnings(workspace);

        // Finalize deps summary from deduped sets; keep non-null for stable schema
        let deps = Some(crate::artifacts::dependencies::DependencySummary {
            nodes: dep_node_keys.len(),
            edges: dep_edge_keys.len(),
        });
        AnalysisReport {
            schema_version: 1,
            diagnostics: merged_diags,
            metrics: merged_metrics,
            cfg: merged_cfg,
            deps,
            workspace_warnings: ws_warnings,
            workspace_errors: Vec::new(),
            deps_node_keys: None,
            deps_edge_keys: None,
        }
    }

    /// Same as `run_workspace` but applies a provided AnalysisConfig to each file's context.
    pub fn run_workspace_with_config(
        workspace: &Workspace,
        config: crate::context::AnalysisConfig,
    ) -> AnalysisReport {
        let mut files: Vec<std::path::PathBuf> = workspace
            .projects
            .iter()
            .flat_map(|p| p.files.iter())
            .filter(|f| matches!(f.kind, ProjectFileKind::Source))
            .map(|f| f.path.clone())
            .collect();
        files.sort();
        files.dedup();

        // Apply workspace include/exclude globs (best effort)
        if !config.workspace.include.is_empty() || !config.workspace.exclude.is_empty() {
            use std::collections::HashSet;
            let mut include_set: Option<HashSet<std::path::PathBuf>> = None;
            if !config.workspace.include.is_empty() {
                let mut set = HashSet::new();
                for pat in &config.workspace.include {
                    if let Ok(walker) =
                        globwalk::GlobWalkerBuilder::new(&workspace.root, pat).build()
                    {
                        for entry in walker.filter_map(Result::ok) {
                            set.insert(entry.path().to_path_buf());
                        }
                    }
                }
                include_set = Some(set);
            }
            let mut exclude_set: HashSet<std::path::PathBuf> = HashSet::new();
            for pat in &config.workspace.exclude {
                if let Ok(walker) = globwalk::GlobWalkerBuilder::new(&workspace.root, pat).build() {
                    for entry in walker.filter_map(Result::ok) {
                        exclude_set.insert(entry.path().to_path_buf());
                    }
                }
            }
            files.retain(|p| {
                let in_includes = include_set.as_ref().map(|s| s.contains(p)).unwrap_or(true);
                let not_excluded = !exclude_set.contains(p);
                in_includes && not_excluded
            });
        }

        let mut merged_diags = DiagnosticCollection::default();
        let mut merged_metrics: Option<AstAnalysis> = None;
        let mut merged_cfg: Option<CfgSummary> = None;
        let mut dep_node_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut dep_edge_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

        #[cfg(not(feature = "parallel_analysis"))]
        for path in files {
            if let Some(report) = Self::analyze_file_report(&path, Some(&config)) {
                Self::merge_report(
                    &mut merged_diags,
                    &mut merged_metrics,
                    &mut merged_cfg,
                    &mut dep_node_keys,
                    &mut dep_edge_keys,
                    &report,
                );
            }
        }

        #[cfg(feature = "parallel_analysis")]
        {
            let results: Vec<(std::path::PathBuf, AnalysisReport)> = files
                .par_iter()
                .filter_map(|path| {
                    Self::analyze_file_report(path, Some(&config))
                        .map(|report| (path.clone(), report))
                })
                .collect();
            // Merge deterministically by sorted path
            let mut sorted = results;
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            for (_path, report) in sorted.into_iter() {
                Self::merge_report(
                    &mut merged_diags,
                    &mut merged_metrics,
                    &mut merged_cfg,
                    &mut dep_node_keys,
                    &mut dep_edge_keys,
                    &report,
                );
            }
        }
        // Finalize deps summary from deduped sets; keep non-null for stable schema
        let deps = Some(crate::artifacts::dependencies::DependencySummary {
            nodes: dep_node_keys.len(),
            edges: dep_edge_keys.len(),
        });
        Self::sort_diagnostics(&mut merged_diags);
        let ws_warnings = Self::collect_workspace_warnings(workspace);
        AnalysisReport {
            schema_version: 1,
            diagnostics: merged_diags,
            metrics: merged_metrics,
            cfg: merged_cfg,
            deps,
            workspace_warnings: ws_warnings,
            workspace_errors: Vec::new(),
            deps_node_keys: None,
            deps_edge_keys: None,
        }
    }
}

impl AnalyzerPipeline {
    pub fn sort_diagnostics(merged_diags: &mut DiagnosticCollection) {
        merged_diags.diagnostics.sort_by(|a, b| {
            let af = a
                .location
                .as_ref()
                .map(|l| l.file.clone())
                .unwrap_or_default();
            let bf = b
                .location
                .as_ref()
                .map(|l| l.file.clone())
                .unwrap_or_default();
            af.cmp(&bf)
                .then_with(|| {
                    a.location
                        .as_ref()
                        .map(|l| l.line)
                        .unwrap_or(0)
                        .cmp(&b.location.as_ref().map(|l| l.line).unwrap_or(0))
                })
                .then_with(|| {
                    a.location
                        .as_ref()
                        .map(|l| l.column)
                        .unwrap_or(0)
                        .cmp(&b.location.as_ref().map(|l| l.column).unwrap_or(0))
                })
                .then_with(|| a.code.as_str().cmp(b.code.as_str()))
        });
    }
}
