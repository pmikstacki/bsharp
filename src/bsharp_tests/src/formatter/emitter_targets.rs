use assert_cmd::prelude::*;
use similar::TextDiff;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::NamedTempFile;
use tempfile::Builder as TempBuilder;
use super::csharpier_installer::ensure_csharpier_bin;

fn run_our_formatter(input: &Path) -> anyhow::Result<String> {
    let output = Command::cargo_bin("bsharp_cli")?
        .args([
            "format",
            input.to_str().unwrap(),
            "--write=false",
            "--print",
            "--newline-mode",
            "lf",
            "--blank-line-between-members",
            "false",
        ])
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "bsharp_cli failed: status={:?}, stderr=\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8(output.stdout)?)
}

#[test]
fn do_while_layout_matches_csharpier() -> anyhow::Result<()> {
    let src = r#"
namespace T {
    public class C {
        public void M(){ int x=0; do { x++; } while (x < 3); }
    }
}
"#;
    let mut tmp = TempBuilder::new().suffix(".cs").tempfile()?;
    tmp.write_all(src.as_bytes())?;
    let path = tmp.path();
    let ours = normalize_eol(run_our_formatter(path)?);
    let theirs = normalize_eol(run_csharpier(path)?);
    if ours != theirs {
        let report = diff_report("ours (bsharp)", &ours, "csharpier", &theirs);
        panic!("Do-while formatting mismatch:\n{}", report);
    }
    Ok(())
}

#[test]
fn switch_layout_matches_csharpier() -> anyhow::Result<()> {
    let src = r#"
namespace T {
    public class C {
        public void M(int x){
            switch (x)
            {
                case 1: x++; break;
                case 2: x += 2; break;
                default: break;
            }
        }
    }
}
"#;
    let mut tmp = TempBuilder::new().suffix(".cs").tempfile()?;
    tmp.write_all(src.as_bytes())?;
    let path = tmp.path();
    let ours = normalize_eol(run_our_formatter(path)?);
    let theirs = normalize_eol(run_csharpier(path)?);
    if ours != theirs {
        let report = diff_report("ours (bsharp)", &ours, "csharpier", &theirs);
        panic!("Switch formatting mismatch:\n{}", report);
    }
    Ok(())
}

#[test]
fn eof_newline_policy_matches_csharpier() -> anyhow::Result<()> {
    let src = "namespace T { public class C { } }   \n\n\n";
    let mut tmp = TempBuilder::new().suffix(".cs").tempfile()?;
    tmp.write_all(src.as_bytes())?;
    let path = tmp.path();
    let ours = normalize_eol(run_our_formatter(path)?);
    let theirs = normalize_eol(run_csharpier(path)?);
    if ours != theirs {
        let report = diff_report("ours (bsharp)", &ours, "csharpier", &theirs);
        panic!("EOF newline policy mismatch:\n{}", report);
    }
    Ok(())
}

fn run_csharpier(input: &Path) -> anyhow::Result<String> {
    let bin = ensure_csharpier_bin()?;
    let status = Command::new(&bin).arg("format").arg(input).status()?;
    if !status.success() {
        anyhow::bail!("csharpier failed to format file: {}", input.display());
    }
    let formatted = std::fs::read_to_string(input)?;
    Ok(formatted)
}

fn normalize_eol(mut s: String) -> String { s = s.replace("\r\n", "\n"); if !s.ends_with('\n') { s.push('\n'); } s }

fn diff_report(left_name: &str, left: &str, right_name: &str, right: &str) -> String {
    let diff = TextDiff::from_lines(left, right);
    let mut s = String::new();
    s.push_str(&format!("--- {}\n", left_name));
    s.push_str(&format!("+++ {}\n", right_name));
    for change in diff.unified_diff().context_radius(3).to_string().lines() {
        s.push_str(change);
        s.push('\n');
    }
    s
}

#[test]
fn event_accessors_format_like_csharpier() -> anyhow::Result<()> {
    let src = r#"
namespace T {
    public class EvtCls {
        public event System.EventHandler SomethingHappened;
        public event System.EventHandler Something
        {
            add { }
            remove { }
        }
    }
}
"#;
    let mut tmp = TempBuilder::new().suffix(".cs").tempfile()?;
    tmp.write_all(src.as_bytes())?;
    let path = tmp.path();
    let ours = normalize_eol(run_our_formatter(path)?);
    let theirs = normalize_eol(run_csharpier(path)?);
    if ours != theirs {
        let report = diff_report("ours (bsharp)", &ours, "csharpier", &theirs);
        panic!("Event accessor formatting mismatch:\n{}", report);
    }
    Ok(())
}

#[test]
fn indexer_format_like_csharpier() -> anyhow::Result<()> {
    let src = r#"
namespace T {
    public class IndexerCls {
        private int[] _a;
        public int this[int idx]
        {
            get { return _a[idx]; }
            set { _a[idx] = value; }
        }
    }
}
"#;
    let mut tmp = TempBuilder::new().suffix(".cs").tempfile()?;
    tmp.write_all(src.as_bytes())?;
    let path = tmp.path();
    let ours = normalize_eol(run_our_formatter(path)?);
    let theirs = normalize_eol(run_csharpier(path)?);
    if ours != theirs {
        let report = diff_report("ours (bsharp)", &ours, "csharpier", &theirs);
        panic!("Indexer formatting mismatch:\n{}", report);
    }
    Ok(())
}

#[test]
#[ignore]
fn operators_format_like_csharpier() -> anyhow::Result<()> {
    let src = r#"
namespace T {
    public class V {
        public int X;
        public V(int x){ X=x; }
        public static V operator -(V a) { return new V(-a.X); }
        public static V operator +(V a, V b) { return new V(a.X + b.X); }
        public static implicit operator int(V a) { return a.X; }
        public static explicit operator V(int v) { return new V(v); }
    }
}
"#;
    let mut tmp = TempBuilder::new().suffix(".cs").tempfile()?;
    tmp.write_all(src.as_bytes())?;
    let path = tmp.path();
    let ours = normalize_eol(run_our_formatter(path)?);
    let theirs = normalize_eol(run_csharpier(path)?);
    if ours != theirs {
        let report = diff_report("ours (bsharp)", &ours, "csharpier", &theirs);
        panic!("Operator formatting mismatch:\n{}", report);
    }
    Ok(())
}
