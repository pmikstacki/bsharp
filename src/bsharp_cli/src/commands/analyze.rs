use anyhow::{Context, Result, bail};
use bsharp_analysis::context::AnalysisContext;
use bsharp_analysis::framework::lookup::find_symbols_with_locations;
use bsharp_analysis::framework::pipeline::AnalyzerPipeline;
use bsharp_analysis::framework::session::AnalysisSession;
use bsharp_analysis::report::AnalysisReport;
use bsharp_analysis::workspace::WorkspaceLoader;
use bsharp_parser::facade::Parser;
use bsharp_syntax::span::Span;
use clap::Args;
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
pub struct AnalyzeArgs {
    /// The input C# file to analyze
    #[arg(required = true)]
    pub input: PathBuf,

    /// Optional symbol name to search for; if omitted, prints all top-level declaration spans
    #[arg(short, long)]
    pub symbol: Option<String>,

    /// Optional analysis config file (JSON/TOML)
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Optional output file path for the analysis JSON report
    #[arg(long, value_name = "FILE")]
    pub out: Option<PathBuf>,

    /// Follow ProjectReference dependencies (default: true)
    #[arg(long, default_value_t = true)]
    pub follow_refs: bool,

    /// Include only files matching these globs (workspace mode). Multiple allowed.
    #[arg(long, value_name = "GLOB", num_args = 0..)]
    pub include: Vec<String>,

    /// Exclude files matching these globs (workspace mode). Multiple allowed.
    #[arg(long, value_name = "GLOB", num_args = 0..)]
    pub exclude: Vec<String>,

    /// Output format: json (compact) or pretty-json (default)
    #[arg(long, value_parser = ["json", "pretty-json"], default_value = "pretty-json")]
    pub format: String,

    /// Enable specific rulesets by id (multiple allowed)
    #[arg(long, value_name = "ID", num_args = 0..)]
    pub enable_ruleset: Vec<String>,

    /// Disable specific rulesets by id (multiple allowed)
    #[arg(long, value_name = "ID", num_args = 0..)]
    pub disable_ruleset: Vec<String>,

    /// Enable specific passes by id (multiple allowed)
    #[arg(long, value_name = "ID", num_args = 0..)]
    pub enable_pass: Vec<String>,

    /// Disable specific passes by id (multiple allowed)
    #[arg(long, value_name = "ID", num_args = 0..)]
    pub disable_pass: Vec<String>,

    /// Override severities: CODE=level (level: error|warning|info|hint); multiple allowed
    #[arg(long, value_name = "PAIR", num_args = 0..)]
    pub severity: Vec<String>,
}

pub fn execute(input: PathBuf, opts: AnalyzeArgs) -> Result<()> {
    let path_str = input
        .to_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| input.display().to_string());

    // Optional: load AnalysisConfig from JSON/TOML
    let loaded_config: Option<bsharp_analysis::context::AnalysisConfig> = match &opts.config {
        Some(cfg_path) => {
            let cfg_str = fs::read_to_string(cfg_path)
                .with_context(|| format!("Failed to read config file: {}", cfg_path.display()))?;
            let ext = cfg_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            let parsed = if ext == "toml" {
                toml::from_str(&cfg_str).map_err(|e| anyhow::anyhow!(e))?
            } else {
                serde_json::from_str(&cfg_str).map_err(|e| anyhow::anyhow!(e))?
            };
            Some(parsed)
        }
        None => None,
    };

    // Workspace mode if input is .sln/.csproj or a directory
    let is_dir = input.is_dir();
    let is_ws = input
        .extension()
        .and_then(|e| e.to_str())
        .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "sln" | "csproj"))
        .unwrap_or(false);

    if is_dir || is_ws {
        let ws = WorkspaceLoader::from_path_with_options(
            &input,
            bsharp_analysis::workspace::loader::WorkspaceLoadOptions {
                follow_refs: opts.follow_refs,
            },
        )
        .map_err(|e| anyhow::anyhow!(e.to_string()))
        .with_context(|| format!("Failed to load workspace from {}", path_str))?;
        // Build final config: prefer file config, override with CLI flags if provided
        let mut cfg = loaded_config.unwrap_or_default();
        cfg.workspace.follow_refs = opts.follow_refs;
        if !opts.include.is_empty() {
            cfg.workspace.include = opts.include.clone();
        }
        if !opts.exclude.is_empty() {
            cfg.workspace.exclude = opts.exclude.clone();
        }
        // Apply toggles: enable overrides disable when both provided
        for id in opts.enable_ruleset {
            cfg.enable_rulesets.insert(id, true);
        }
        for id in opts.disable_ruleset {
            cfg.enable_rulesets.insert(id, false);
        }
        for id in opts.enable_pass {
            cfg.enable_passes.insert(id, true);
        }
        for id in opts.disable_pass {
            cfg.enable_passes.insert(id, false);
        }
        // Severities: parse CODE=level
        for pair in opts.severity {
            if let Some((code, lvl)) = pair.split_once('=') {
                let sev = match lvl.to_ascii_lowercase().as_str() {
                    "error" => bsharp_analysis::DiagnosticSeverity::Error,
                    "warning" => bsharp_analysis::DiagnosticSeverity::Warning,
                    "info" => bsharp_analysis::DiagnosticSeverity::Info,
                    "hint" => bsharp_analysis::DiagnosticSeverity::Hint,
                    _ => continue,
                };
                cfg.rule_severities.insert(code.to_string(), sev);
            }
        }

        let report = AnalyzerPipeline::run_workspace_with_config(&ws, cfg);
        let json = if opts.format == "json" {
            serde_json::to_string(&report)
        } else {
            serde_json::to_string_pretty(&report)
        }
        .with_context(|| "Failed to serialize analysis report to JSON")?;
        if let Some(out_path) = opts.out {
            fs::write(&out_path, json)
                .with_context(|| format!("Failed to write report to {}", out_path.display()))?;
        } else {
            println!("{}", json);
        }
        return Ok(());
    }

    // Single-file mode (legacy behavior, supports --symbol)
    let source = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read input file: {}", path_str))?;
    let parser = Parser::new();
    let (cu, db) = parser
        .parse_with_span_db(Span::new(source.as_str()))
        .map_err(|e| anyhow::anyhow!(e))
        .with_context(|| "Parse error")?;
    let mut ctx = AnalysisContext::new(path_str.clone(), source);
    if let Some(mut cfg) = loaded_config.clone() {
        cfg.workspace.follow_refs = opts.follow_refs;
        if !opts.include.is_empty() {
            cfg.workspace.include = opts.include.clone();
        }
        if !opts.exclude.is_empty() {
            cfg.workspace.exclude = opts.exclude.clone();
        }
        for id in opts.enable_ruleset.clone() {
            cfg.enable_rulesets.insert(id, true);
        }
        for id in opts.disable_ruleset.clone() {
            cfg.enable_rulesets.insert(id, false);
        }
        for id in opts.enable_pass.clone() {
            cfg.enable_passes.insert(id, true);
        }
        for id in opts.disable_pass.clone() {
            cfg.enable_passes.insert(id, false);
        }
        for pair in opts.severity.clone() {
            if let Some((code, lvl)) = pair.split_once('=') {
                let sev = match lvl.to_ascii_lowercase().as_str() {
                    "error" => bsharp_analysis::DiagnosticSeverity::Error,
                    "warning" => bsharp_analysis::DiagnosticSeverity::Warning,
                    "info" => bsharp_analysis::DiagnosticSeverity::Info,
                    "hint" => bsharp_analysis::DiagnosticSeverity::Hint,
                    _ => continue,
                };
                cfg.rule_severities.insert(code.to_string(), sev);
            }
        }
        ctx.config = cfg;
    }

    match opts.symbol {
        Some(name) => {
            // Build a session and run the pipeline to populate SymbolIndex, then query it.
            let mut session = AnalysisSession::new(ctx.clone(), Default::default());
            session.span_db = Some(db.clone());
            AnalyzerPipeline::run_with_defaults(&cu, &mut session);
            let results = find_symbols_with_locations(&session, &name);
            if results.is_empty() {
                bail!("No declarations found for symbol '{}'.", name);
            }
            println!("Found {} result(s) for symbol '{}':", results.len(), name);
            for (sym, loc) in results {
                let kind = format!("{:?}", sym.kind);
                if let Some(loc) = loc {
                    println!("- {} {} at {}:{}", kind, sym.name, loc.line, loc.column);
                } else if let Some(file) = sym.file {
                    println!("- {} {} in {} (no precise location)", kind, sym.name, file);
                } else {
                    println!("- {} {} (no location)", kind, sym.name);
                }
            }
        }
        None => {
            let mut session = AnalysisSession::new(ctx.clone(), Default::default());
            session.span_db = Some(db);
            AnalyzerPipeline::run_with_defaults(&cu, &mut session);
            let report = session
                .artifacts
                .get::<AnalysisReport>()
                .map(|a| (*a).clone())
                .unwrap_or_else(|| AnalysisReport::from_session(&session));
            let json = if opts.format == "json" {
                serde_json::to_string(&report)
            } else {
                serde_json::to_string_pretty(&report)
            }
            .with_context(|| "Failed to serialize analysis report to JSON")?;
            if let Some(out_path) = opts.out {
                fs::write(&out_path, json)
                    .with_context(|| format!("Failed to write report to {}", out_path.display()))?;
            } else {
                println!("{}", json);
            }
        }
    }

    Ok(())
}
