use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

pub fn resolve_csharpier_path() -> Option<String> {
    // Prefer PATH first
    if Command::new("csharpier").arg("--version").output().is_ok() {
        return Some("csharpier".to_string());
    }
    // Try ~/.dotnet/tools/csharpier
    if let Ok(home) = env::var("HOME") {
        let candidate = format!("{}/.dotnet/tools/csharpier", home);
        if fs::metadata(&candidate).is_ok() {
            return Some(candidate);
        }
    }
    // Common macOS locations
    for p in ["/usr/local/bin/csharpier", "/opt/homebrew/bin/csharpier"] {
        if fs::metadata(p).is_ok() {
            return Some(p.to_string());
        }
    }
    None
}

pub fn ensure_csharpier_bin() -> anyhow::Result<String> {
    if let Some(bin) = resolve_csharpier_path() {
        return Ok(bin);
    }
    // Optional local install (opt-in via env)
    let auto = env::var("BSHARP_AUTO_INSTALL_CSHARPIER").unwrap_or_default();
    let auto = matches!(auto.as_str(), "1" | "true" | "TRUE" | "yes");
    if auto {
        let tool_dir: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "..",
            "..",
            "target",
            "tools",
            "csharpier_bin",
        ]
        .iter()
        .collect();
        fs::create_dir_all(&tool_dir)?;
        let status = Command::new("dotnet")
            .args([
                "tool",
                "install",
                "--tool-path",
                tool_dir.to_str().unwrap(),
                "csharpier",
            ])
            .status()?;
        if !status.success() {
            anyhow::bail!("failed to install csharpier with dotnet tool");
        }
        let bin = tool_dir.join("csharpier");
        if fs::metadata(&bin).is_ok() {
            return Ok(bin.to_string_lossy().to_string());
        }
    }
    anyhow::bail!(
        "CSharpier not found. Install globally: `dotnet tool install -g csharpier` or set BSHARP_AUTO_INSTALL_CSHARPIER=1 to auto-install locally."
    )
}
