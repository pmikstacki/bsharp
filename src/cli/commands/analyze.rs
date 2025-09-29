use anyhow::{bail, Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::analysis::context::AnalysisContext;
use crate::analysis::navigation::find_by_name_with_context;
use crate::analysis::framework::pipeline::AnalyzerPipeline;
use crate::analysis::framework::session::AnalysisSession;
use crate::analysis::report::AnalysisReport;
use crate::syntax::Parser;
use crate::workspace::WorkspaceLoader;
use serde_json;

pub fn execute(
    input: PathBuf,
    symbol: Option<String>,
    config: Option<PathBuf>,
    out: Option<PathBuf>,
    follow_refs: bool,
    include: Vec<String>,
    exclude: Vec<String>,
    format: String,
) -> Result<()> {
    let path_str = input
        .to_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| input.display().to_string());

    // Optional: load AnalysisConfig from JSON/TOML
    let loaded_config: Option<crate::analysis::context::AnalysisConfig> = match &config {
        Some(cfg_path) => {
            let cfg_str = fs::read_to_string(cfg_path)
                .with_context(|| format!("Failed to read config file: {}", cfg_path.display()))?;
            let ext = cfg_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
            let parsed = if ext == "toml" { toml::from_str(&cfg_str).map_err(|e| anyhow::anyhow!(e))? } else { serde_json::from_str(&cfg_str).map_err(|e| anyhow::anyhow!(e))? };
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
        let ws = WorkspaceLoader::from_path(&input)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .with_context(|| format!("Failed to load workspace from {}", path_str))?;
        // Build final config: prefer file config, override with CLI flags if provided
        let mut cfg = loaded_config.unwrap_or_default();
        cfg.workspace.follow_refs = follow_refs;
        if !include.is_empty() { cfg.workspace.include = include.clone(); }
        if !exclude.is_empty() { cfg.workspace.exclude = exclude.clone(); }

        let report = AnalyzerPipeline::run_workspace_with_config(&ws, cfg);
        let json = if format == "json" { serde_json::to_string(&report) } else { serde_json::to_string_pretty(&report) }
            .with_context(|| "Failed to serialize analysis report to JSON")?;
        if let Some(out_path) = out {
            fs::write(&out_path, json).with_context(|| format!("Failed to write report to {}", out_path.display()))?;
        } else {
            println!("{}", json);
        }
        return Ok(());
    }

    // Single-file mode (legacy behavior, supports --symbol)
    let source = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read input file: {}", path_str))?;
    let parser = Parser::new();
    let (cu, spans) = parser
        .parse_with_spans(&source)
        .map_err(|e| anyhow::anyhow!(e))
        .with_context(|| "Parse error")?;
    let mut ctx = AnalysisContext::new(path_str.clone(), source);
    if let Some(mut cfg) = loaded_config.clone() {
        cfg.workspace.follow_refs = follow_refs;
        if !include.is_empty() { cfg.workspace.include = include.clone(); }
        if !exclude.is_empty() { cfg.workspace.exclude = exclude.clone(); }
        ctx.config = cfg;
    }

    match symbol {
        Some(name) => {
            let results = find_by_name_with_context(&cu, &name, &ctx, &spans);
            if results.is_empty() {
                bail!("No declarations found for symbol '{}'.", name);
            }
            println!("Found {} result(s) for symbol '{}':", results.len(), name);
            for info in results {
                let kind = format!("{:?}", info.declaration_type);
                if let Some(loc) = info.location {
                    println!("- {} {} at {}:{} (len {})", kind, info.name, loc.line, loc.column, loc.length);
                } else {
                    println!("- {} {} (no location)", kind, info.name);
                }
            }
        }
        None => {
            let mut session = AnalysisSession::new(ctx.clone(), spans.clone());
            AnalyzerPipeline::run_with_defaults(&cu, &mut session);
            let report = session
                .artifacts
                .get::<AnalysisReport>()
                .map(|a| (*a).clone())
                .unwrap_or_else(|| AnalysisReport::from_session(&session));
            let json = if format == "json" { serde_json::to_string(&report) } else { serde_json::to_string_pretty(&report) }
                .with_context(|| "Failed to serialize analysis report to JSON")?;
            if let Some(out_path) = out {
                fs::write(&out_path, json).with_context(|| format!("Failed to write report to {}", out_path.display()))?;
            } else {
                println!("{}", json);
            }
        }
    }

    Ok(())
}
