use super::csharpier_installer::ensure_csharpier_bin;
use assert_cmd::prelude::*;
use parser::bsharp::parse_csharp_source_strict;
use parser::syntax::span::Span;
use similar::TextDiff;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, io};
use tempfile::Builder as TempBuilder;
use tempfile::NamedTempFile;

fn run_our_formatter(input: &PathBuf) -> anyhow::Result<String> {
    // Use the CLI binary to format and print to stdout without writing
    let output = Command::cargo_bin("bsharp_cli")?
        .args([
            "format",
            input.to_str().unwrap(),
            "--write=false",
            "--print",
            "--newline-mode",
            "lf",
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

fn run_csharpier(input: &PathBuf) -> anyhow::Result<String> {
    // Do not mutate fixtures: copy to a temp file, run csharpier in-place, read back
    let src = fs::read_to_string(input)?;
    let mut tmp = TempBuilder::new().suffix(".cs").tempfile()?;
    use std::io::Write as _;
    tmp.write_all(src.as_bytes())?;
    let tmp_path = tmp.path();
    let bin = ensure_csharpier_bin()?;
    let status = Command::new(&bin).arg("format").arg(tmp_path).status()?;
    if !status.success() {
        anyhow::bail!("csharpier failed to format file: {}", input.display());
    }
    let formatted = fs::read_to_string(tmp_path)?;
    Ok(formatted)
}

fn collect_cs_files(dir: &PathBuf, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let e = entry?;
        let p = e.path();
        if e.file_type()?.is_dir() {
            collect_cs_files(&p, out)?;
        } else if p
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("cs"))
            .unwrap_or(false)
        {
            out.push(p);
        }
    }
    Ok(())
}

fn normalize_eol(mut s: String) -> String {
    s = s.replace("\r\n", "\n");
    if !s.ends_with('\n') {
        s.push('\n');
    }
    s
}

#[test]
fn cs_test_cases_match_csharpier_suite() -> anyhow::Result<()> {
    // Locate directory
    let dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "src", "cs_test_cases"]
        .iter()
        .collect();

    let mut files = Vec::new();
    collect_cs_files(&dir, &mut files)?;

    let mut mismatches: Vec<(PathBuf, String)> = Vec::new();

    for file in files {
        // Skip files our parser cannot handle yet
        let src = fs::read_to_string(&file)?;
        if parse_csharp_source_strict(Span::new(&src)).is_err() {
            continue;
        }

        let csharpier = match run_csharpier(&file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("skipping (csharpier failed): {} -> {}", file.display(), e);
                continue;
            }
        };
        let ours = run_our_formatter(&file)?;
        let left = normalize_eol(ours);
        let right = normalize_eol(csharpier);
        if left != right {
            let report = diff_report("ours (bsharp)", &left, "csharpier", &right);
            mismatches.push((file.clone(), report));
        }
    }

    if !mismatches.is_empty() {
        let mut msg = String::new();
        msg.push_str(&format!(
            "{} mismatches found vs CSharpier:\n\n",
            mismatches.len()
        ));
        for (i, (path, rep)) in mismatches.iter().enumerate() {
            if i >= 5 {
                msg.push_str("... (truncated)\n");
                break;
            }
            msg.push_str(&format!("File: {}\n{}\n", path.display(), rep));
        }
        panic!("{}", msg);
    }
    Ok(())
}

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
fn advanced_features_matches_csharpier() -> anyhow::Result<()> {
    // Arrange
    let file: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "src",
        "cs_test_cases",
        "advanced_features.cs",
    ]
    .iter()
    .collect();

    let csharpier = run_csharpier(&file)?;

    let ours = run_our_formatter(&file)?;

    if ours != csharpier {
        let report = diff_report("ours (bsharp)", &ours, "csharpier", &csharpier);
        panic!(
            "Formatter mismatch with CSharpier for {}:\n{}",
            file.display(),
            report
        );
    }
    Ok(())
}
